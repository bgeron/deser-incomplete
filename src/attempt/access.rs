#[cfg(doc)]
use serde::de::Deserializer;
use serde::de::{EnumAccess, MapAccess, SeqAccess, VariantAccess};

use crate::options_impl::ExtraOptions;
use crate::reporter::Reporter;
use crate::state::InterventionReason;
use crate::util::DeserializeKind;

use super::visit::Visitor;
use super::{erase_error_ref, AttemptState, GlobalState, HaltingPoint, InnerDeserializeSeed};

pub(crate) struct Access<'a, Inner, Extra>
where
    Extra: ExtraOptions,
{
    pub(crate) global: &'a mut GlobalState<Extra>,
    pub(crate) attempt: &'a mut AttemptState<Extra>,
    pub(crate) kind: DeserializeKind,
    pub(crate) inner: Inner,
    pub(crate) collection_has_ended: bool,
    pub(crate) inside_element: Option<InsideElement>,
}

#[derive(Debug)]
pub(crate) struct InsideElement {
    pub(crate) corresponding_halting_point: HaltingPoint,
    pub(crate) halting_point_is_on_stack: bool,
}

impl<Inner, Extra> Access<'_, Inner, Extra>
where
    Extra: ExtraOptions,
{
    /// Manage halting points for access types that allow backtracking of elements.
    fn enter_element(&mut self, corresponding_halting_point: HaltingPoint) {
        trace!(%corresponding_halting_point, ?self.inside_element, "entering");
        if self.inside_element.is_some() {
            error!(
                "access: enter before leaving previous element (maybe next_value_seed was not called?)"
            );
        }

        let halting_point_is_on_stack = self.should_backtrack_skip_item();

        if halting_point_is_on_stack {
            self.attempt
                .halting_point_stack
                .push(corresponding_halting_point.clone());
        }

        self.inside_element = Some(InsideElement {
            corresponding_halting_point,
            halting_point_is_on_stack,
        });
    }

    fn leave_element(&mut self) {
        trace!(?self.inside_element, ?self.attempt.halting_point_stack, "leaving");
        let Some(inside_element) = self.inside_element.take() else {
            error!("access: leave without entering element (maybe next_key_seed was not called?)");
            return;
        };

        if inside_element.halting_point_is_on_stack {
            loop {
                let last_point = self
                    .attempt
                    .halting_point_stack
                    .pop()
                    .expect("halting point unexpectedly disappeared from stack");
                if last_point == inside_element.corresponding_halting_point {
                    break;
                }
            }
        }
    }
}

impl<Inner, Extra> Access<'_, Inner, Extra>
where
    Extra: ExtraOptions,
{
    /// Determine whether it's okay to fallback to skipping an item,
    /// depending on what we are deserializing for (the [`Deserializer`] method)
    pub(crate) fn should_fallback_skip_item(&self) -> bool {
        match self.kind {
            DeserializeKind::Seq => self.global.config.behavior.fallback_seq_skip_item,
            // Tuples don't have optional elements.
            DeserializeKind::Tuple { len: _ } => false,
            // Tuple structs may have optional fields.
            DeserializeKind::TupleStruct { name: _, len: _ } => {
                self.global.config.behavior.fallback_seq_skip_item
            }
            DeserializeKind::Map => self.global.config.behavior.fallback_map_skip_item,
            DeserializeKind::Struct { name: _, fields: _ } => {
                self.global.config.behavior.fallback_struct_skip_field
            }
            DeserializeKind::Enum { .. } => false,

            _ => self.global.config.behavior.fallback_other_skip_item,
        }
    }

    /// Determine whether it's okay to backtrack skipping an item
    pub(crate) fn should_backtrack_skip_item(&self) -> bool {
        match self.kind {
            DeserializeKind::Seq => self.global.config.behavior.backtrack_seq_skip_item,
            DeserializeKind::Tuple { len: _ } => false,
            DeserializeKind::TupleStruct { name: _, len: _ } => {
                self.global.config.behavior.backtrack_seq_skip_item
            }
            DeserializeKind::Map => self.global.config.behavior.backtrack_map_skip_item,
            DeserializeKind::Struct { name: _, fields: _ } => {
                self.global.config.behavior.backtrack_struct_skip_field
            }
            DeserializeKind::Enum { .. } => false,

            _ => self.global.config.behavior.backtrack_other_skip_item,
        }
    }
}

impl<'de, Inner, Extra> SeqAccess<'de> for Access<'_, Inner, Extra>
where
    Inner: SeqAccess<'de>,
    Extra: ExtraOptions,
{
    type Error = Inner::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.collection_has_ended {
            self.global.reporter.report_access_past_end();
        }
        let Some(this_halting_point) = self
            .attempt
            .intervention_is_empty()
            .then(|| self.attempt.new_halting_point_and_check_continue())
            .flatten()
        else {
            self.global.reporter.report_seq_next_element_skip();
            self.collection_has_ended = true;
            return Ok(None);
        };

        self.enter_element(this_halting_point);

        self.global.reporter.report_seq_next_element_start();
        let wrapped_seed = InnerDeserializeSeed {
            global: self.global,
            attempt: self.attempt,
            is_for_key_or_variant: false,
            is_for_map_value: false,
            inner: seed,
        };
        let result = self.inner.next_element_seed(wrapped_seed);
        self.global.reporter.report_seq_next_element_finish(
            matches!(result, Ok(Some(_))),
            erase_error_ref(&result),
        );
        if result.is_err() {
            self.attempt
                .activate_intervention(InterventionReason::VisitError);
        }
        self.leave_element();

        match result {
            Ok(Some(v)) => Ok(Some(v)),
            Ok(None) => {
                self.collection_has_ended = true;
                Ok(None)
            }
            Err(_) if self.should_fallback_skip_item() => {
                self.global.reporter.report_fallback_no_element();
                self.collection_has_ended = true;
                Ok(None)
            }
            Err(e) => {
                self.collection_has_ended = true;
                Err(e)
            }
        }
    }
}

impl<'de, Inner, Extra> MapAccess<'de> for Access<'_, Inner, Extra>
where
    Inner: MapAccess<'de>,
    Extra: ExtraOptions,
{
    type Error = Inner::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.collection_has_ended {
            self.global.reporter.report_access_past_end();
        }
        let Some(this_halting_point) = self
            .attempt
            .intervention_is_empty()
            .then(|| self.attempt.new_halting_point_and_check_continue())
            .flatten()
        else {
            self.global.reporter.report_map_next_key_skip();
            self.collection_has_ended = true;
            return Ok(None);
        };

        self.enter_element(this_halting_point);

        self.global.reporter.report_map_next_key_start();
        let wrapped_seed = InnerDeserializeSeed {
            global: self.global,
            attempt: self.attempt,
            is_for_key_or_variant: true,
            is_for_map_value: false,
            inner: seed,
        };
        let result = self.inner.next_key_seed(wrapped_seed);
        self.global
            .reporter
            .report_map_next_key_finish(matches!(result, Ok(Some(_))), erase_error_ref(&result));
        if result.is_err() {
            self.attempt
                .activate_intervention(InterventionReason::VisitError);
        }
        if !matches!(result, Ok(Some(_))) {
            self.leave_element();
        }

        match result {
            Ok(Some(v)) => Ok(Some(v)),
            Ok(None) => {
                self.collection_has_ended = true;
                Ok(None)
            }
            Err(_) if self.should_fallback_skip_item() => {
                self.global.reporter.report_fallback_no_element();
                self.collection_has_ended = true;
                Ok(None)
            }
            Err(e) => {
                self.collection_has_ended = true;
                Err(e)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if self.collection_has_ended {
            self.global.reporter.report_access_past_end();
        }

        self.global.reporter.report_map_next_value_start();
        let wrapped_seed = InnerDeserializeSeed {
            global: self.global,
            attempt: self.attempt,
            is_for_key_or_variant: false,
            is_for_map_value: true,
            inner: seed,
        };
        let result = self.inner.next_value_seed(wrapped_seed);
        self.global
            .reporter
            .report_map_next_value_finish(erase_error_ref(&result));

        if result.is_err() {
            self.collection_has_ended = true;
            self.attempt
                .activate_intervention(InterventionReason::VisitError);
        }
        self.leave_element();

        result
    }
}

impl<'a, 'de, Inner, Extra> EnumAccess<'de> for Access<'a, Inner, Extra>
where
    Inner: EnumAccess<'de>,
    Extra: ExtraOptions,
{
    type Error = Inner::Error;
    type Variant = Access<'a, Inner::Variant, Extra>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.global.reporter.report_enum_start();

        let result = self.inner.variant_seed(InnerDeserializeSeed {
            global: self.global,
            attempt: self.attempt,
            is_for_key_or_variant: true,
            is_for_map_value: false,
            inner: seed,
        });
        self.global
            .reporter
            .report_enum_finish(erase_error_ref(&result));
        if result.is_err() {
            self.attempt
                .activate_intervention(InterventionReason::VisitError);
        }

        let (value, inner_variant) = result?;
        Ok((
            value,
            Access {
                global: self.global,
                attempt: self.attempt,
                kind: self.kind,
                inner: inner_variant,
                collection_has_ended: false,
                inside_element: None,
            },
        ))
    }
}

fn process_variant_result<T, E: std::error::Error>(
    result: &Result<T, E>,
    attempt: &mut AttemptState<impl ExtraOptions>,
    reporter: &mut impl Reporter,
) {
    if result.is_err() {
        attempt.activate_intervention(InterventionReason::VisitError);
    }
    reporter.report_variant_finish(erase_error_ref(result));
}

impl<'de, Inner, Extra> VariantAccess<'de> for Access<'_, Inner, Extra>
where
    Inner: VariantAccess<'de>,
    Extra: ExtraOptions,
{
    type Error = Inner::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.global.reporter.report_variant_start_unit_variant();
        let result = self.inner.unit_variant();
        if result.is_err() {
            self.attempt
                .activate_intervention(InterventionReason::VisitError);
        }
        process_variant_result(&result, self.attempt, &mut self.global.reporter);
        match result {
            Ok(()) => Ok(()),
            Err(_) if self.global.config.behavior.fallback_unit_variant => {
                self.global.reporter.report_fallback(None);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.global.reporter.report_variant_start_newtype_variant();
        let result = self.inner.newtype_variant_seed(InnerDeserializeSeed {
            global: self.global,
            attempt: self.attempt,
            is_for_key_or_variant: false,
            is_for_map_value: true,
            inner: seed,
        });
        process_variant_result(&result, self.attempt, &mut self.global.reporter);
        result
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = Some(visitor);
        let mut value = None;

        self.global.reporter.report_variant_start_tuple_variant(len);
        let wrapped_visitor = Visitor {
            global: self.global,
            attempt: self.attempt,
            kind: self.kind,
            is_at_root: false,
            is_for_key_or_variant: false,
            inner: &mut visitor,
            value: &mut value,
        };
        let result = self.inner.tuple_variant(len, wrapped_visitor);
        process_variant_result(&result, self.attempt, &mut self.global.reporter);

        match result {
            Ok(_) => Ok(value.expect("successful visitor will place its value")),
            Err(e) => Err(e),
        }
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = Some(visitor);
        let mut value = None;

        self.global
            .reporter
            .report_variant_start_struct_variant(fields);
        let wrapped_visitor = Visitor {
            global: self.global,
            attempt: self.attempt,
            kind: self.kind,
            is_at_root: false,
            is_for_key_or_variant: false,
            inner: &mut visitor,
            value: &mut value,
        };
        let result = self.inner.struct_variant(fields, wrapped_visitor);
        process_variant_result(&result, self.attempt, &mut self.global.reporter);

        match result {
            Ok(_) => Ok(value.expect("successful visitor will place its value")),
            Err(e) => Err(e),
        }
    }
}
