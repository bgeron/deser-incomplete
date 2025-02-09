use super::empty_access::EmptyAccess;
use super::visit::Visitor;
use super::Deserializer;
use crate::error::{BugEnum, Error, FallbackError};
use crate::fallback::{FallbackContext, FallbacksExt as _};
use crate::options_impl::ExtraOptions;
use crate::reporter::{self, Reporter, ReporterExt as _};
use crate::state::InterventionReason;
use crate::util::{erase_error_ref, make_fnonce, DeserializeKind};

fn framework<'a, 'de, InnerDeserializer, Extra, InnerVisitor>(
    deserializer: Deserializer<'a, InnerDeserializer, Extra>,
    inner_visitor: InnerVisitor,
    kind: DeserializeKind,
    deserialize_method: impl FnOnce(
        InnerDeserializer,
        Visitor<'_, 'de, InnerVisitor, Extra>,
    ) -> Result<(), InnerDeserializer::Error>,
) -> Result<InnerVisitor::Value, Error<InnerDeserializer::Error>>
where
    Extra: ExtraOptions,
    InnerDeserializer: serde::Deserializer<'de>,
    InnerVisitor: serde::de::Visitor<'de>,
{
    let report_args = reporter::DeserializeStartArgsImpl {
        visitor: &inner_visitor,
    };
    deserializer
        .global
        .reporter
        .report_deserialize_start(report_args, kind);

    // The wrapped visitor will place a successful value here.
    let mut value: Option<InnerVisitor::Value> = None;
    // If the deserializer actually tries to visit, then this will be consumed.
    // Otherwise we will keep it, and try to visit with a callback.
    let mut visitor = Some(inner_visitor);

    let result = deserialize_method(
        deserializer.inner,
        Visitor {
            global: deserializer.global,
            attempt: deserializer.attempt,
            kind,
            is_at_root: deserializer.is_at_root,
            is_for_key_or_variant: deserializer.is_for_key_or_variant,
            inner: &mut visitor,
            value: &mut value,
        },
    );
    deserializer
        .global
        .reporter
        .report_deserialize_finish(erase_error_ref(&result));
    if result.is_err() {
        deserializer
            .attempt
            .activate_intervention(InterventionReason::DeserializerStart);
    }

    if result.is_err() && visitor.is_some() {
        // We can try to apply a fallback.
        deserializer.global.reporter.report_start_fallback();
        let context = FallbackContext {
            is_at_root: deserializer.is_at_root,
            is_for_map_value: deserializer.is_for_map_value,
        };
        let take_visitor =
            make_fnonce(|| visitor.take().expect("a Some can be .take()n in an FnOnce"));
        let result_opt = match deserializer
            .global
            .fallbacks
            .fallback(&context, take_visitor, kind)
        {
            Ok(Some(value)) => Some(Ok(value)),
            Err(err) => Some(Err(FallbackError::FallbackVisitor(err))),
            Ok(None) if visitor.is_some() => None,
            Ok(None) => Some(Err(FallbackError::FallbackDidntCompute)),
        };

        if let Some(result) = result_opt {
            deserializer
                .global
                .reporter
                .report_fallback(erase_error_ref(&result));

            if let Ok(value) = result {
                return Ok(value);
            }
        } else {
            // The fallback didn't try to compute a value
            deserializer.global.reporter.report_no_fallback();
        }
    }

    match (result, value) {
        (Ok(()), Some(value)) => Ok(value),
        (Ok(()), None) => {
            error!("internal error: our visitor seems to have succeeded, but not saved its value");
            Err(BugEnum::OkButValueMissingFromStack.into())
        }
        (Err(_), Some(value))
            if deserializer
                .global
                .config
                .behavior
                .tolerate_deserializer_fail_after_visit_success =>
        {
            deserializer
                .attempt
                .activate_intervention(InterventionReason::DeserializerFinishSaved);
            deserializer
                .global
                .reporter
                .report_deserialize_fallback_use_saved_value();
            Ok(value)
        }
        (Err(e), _) => Err(Error::from_de(e)),
    }
}

impl<'de, Inner, Extra> serde::Deserializer<'de> for Deserializer<'_, Inner, Extra>
where
    Inner: serde::Deserializer<'de>,
    Extra: ExtraOptions,
{
    type Error = Error<Inner::Error>;

    fn deserialize_any<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::Any,
            |inner, visitor| inner.deserialize_any(visitor),
        )
    }

    fn deserialize_bool<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::Bool,
            |inner, visitor| inner.deserialize_bool(visitor),
        )
    }

    fn deserialize_i8<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::I8,
            |inner, visitor| inner.deserialize_i8(visitor),
        )
    }

    fn deserialize_i16<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::I16,
            |inner, visitor| inner.deserialize_i16(visitor),
        )
    }

    fn deserialize_i32<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::I32,
            |inner, visitor| inner.deserialize_i32(visitor),
        )
    }

    fn deserialize_i64<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::I64,
            |inner, visitor| inner.deserialize_i64(visitor),
        )
    }

    fn deserialize_i128<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::I128,
            |inner, visitor| inner.deserialize_i128(visitor),
        )
    }

    fn deserialize_u8<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::U8,
            |inner, visitor| inner.deserialize_u8(visitor),
        )
    }

    fn deserialize_u16<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::U16,
            |inner, visitor| inner.deserialize_u16(visitor),
        )
    }

    fn deserialize_u32<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::U32,
            |inner, visitor| inner.deserialize_u32(visitor),
        )
    }

    fn deserialize_u64<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::U64,
            |inner, visitor| inner.deserialize_u64(visitor),
        )
    }

    fn deserialize_u128<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::U128,
            |inner, visitor| inner.deserialize_u128(visitor),
        )
    }

    fn deserialize_f32<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::F32,
            |inner, visitor| inner.deserialize_f32(visitor),
        )
    }

    fn deserialize_f64<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::F64,
            |inner, visitor| inner.deserialize_f64(visitor),
        )
    }

    fn deserialize_char<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::Char,
            |inner, visitor| inner.deserialize_char(visitor),
        )
    }

    fn deserialize_str<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::Str,
            |inner, visitor| inner.deserialize_str(visitor),
        )
    }

    fn deserialize_string<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::String,
            |inner, visitor| inner.deserialize_string(visitor),
        )
    }

    fn deserialize_bytes<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::Bytes,
            |inner, visitor| inner.deserialize_bytes(visitor),
        )
    }

    fn deserialize_byte_buf<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::ByteBuf,
            |inner, visitor| inner.deserialize_byte_buf(visitor),
        )
    }

    fn deserialize_option<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::Option,
            |inner, visitor| inner.deserialize_option(visitor),
        )
    }

    fn deserialize_unit<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::Unit,
            |inner, visitor| inner.deserialize_unit(visitor),
        )
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        inner_visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::UnitStruct { name },
            |inner, visitor| inner.deserialize_unit_struct(name, visitor),
        )
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        inner_visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::NewtypeStruct { name },
            |inner, visitor| inner.deserialize_newtype_struct(name, visitor),
        )
    }

    fn deserialize_seq<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let Some(this_halting_point) = self
            .attempt
            .intervention_is_empty()
            .then(|| self.attempt.new_halting_point_and_check_continue())
            .flatten()
        else {
            return inner_visitor.visit_seq(EmptyAccess::default());
        };

        if self.is_for_map_value && self.global.config.behavior.backtrack_seq_empty_for_value {
            self.attempt.halting_point_stack.push(this_halting_point);
        }

        framework(
            self,
            inner_visitor,
            DeserializeKind::Seq,
            |inner, visitor| inner.deserialize_seq(visitor),
        )
    }

    fn deserialize_tuple<V>(self, len: usize, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::Tuple { len },
            |inner, visitor| inner.deserialize_tuple(len, visitor),
        )
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        inner_visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::TupleStruct { name, len },
            |inner, visitor| inner.deserialize_tuple_struct(name, len, visitor),
        )
    }

    fn deserialize_map<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let Some(this_halting_point) = self
            .attempt
            .intervention_is_empty()
            .then(|| self.attempt.new_halting_point_and_check_continue())
            .flatten()
        else {
            return inner_visitor.visit_map(EmptyAccess::default());
        };

        if self.is_for_map_value && self.global.config.behavior.backtrack_map_empty_for_value {
            self.attempt.halting_point_stack.push(this_halting_point);
        }

        framework(
            self,
            inner_visitor,
            DeserializeKind::Map,
            |inner, visitor| inner.deserialize_map(visitor),
        )
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        inner_visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let Some(this_halting_point) = self
            .attempt
            .intervention_is_empty()
            .then(|| self.attempt.new_halting_point_and_check_continue())
            .flatten()
        else {
            return inner_visitor.visit_map(EmptyAccess::default());
        };

        if self.is_for_map_value && self.global.config.behavior.backtrack_struct_empty_for_value {
            self.attempt.halting_point_stack.push(this_halting_point);
        }

        framework(
            self,
            inner_visitor,
            DeserializeKind::Struct { name, fields },
            |inner, visitor| inner.deserialize_struct(name, fields, visitor),
        )
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        inner_visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::Enum { name, variants },
            |inner, visitor| inner.deserialize_enum(name, variants, visitor),
        )
    }

    fn deserialize_identifier<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::Identifier,
            |inner, visitor| inner.deserialize_identifier(visitor),
        )
    }

    fn deserialize_ignored_any<V>(self, inner_visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        framework(
            self,
            inner_visitor,
            DeserializeKind::IgnoredAny,
            |inner, visitor| inner.deserialize_ignored_any(visitor),
        )
    }

    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
}
