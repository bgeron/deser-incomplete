use std::cell::{Cell, RefCell};
use std::error::Error as StdError;
use std::fmt::Debug;
use std::rc::Rc;

use crate::attempt::HaltingPoint;

use super::{DeserializeStartArgs, DeserializeStartArgsExt, Reporter};

/// A reporter that logs on tracing (if that crate is enabled), or does
/// nothing otherwise.
#[derive(Debug, Default, Clone)]
pub struct DefaultReporter {
    /// nesting level
    level: Rc<Cell<usize>>,
}

impl DefaultReporter {
    pub fn new() -> Self {
        Self::default()
    }

    fn decrease_level(&self) {
        self.level.set(self.level.get() - 1);
    }

    fn increase_level(&self) {
        self.level.set(self.level.get() + 1);
    }
}

impl Reporter for DefaultReporter {
    fn report_deserialize_start_any(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_any");
    }

    fn report_deserialize_start_bool(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_bool");
    }

    fn report_deserialize_start_i8(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_i8");
    }

    fn report_deserialize_start_i16(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_i16");
    }

    fn report_deserialize_start_i32(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_i32");
    }

    fn report_deserialize_start_i64(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_i64");
    }

    fn report_deserialize_start_i128(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_i128");
    }

    fn report_deserialize_start_u8(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_u8");
    }

    fn report_deserialize_start_u16(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_u16");
    }

    fn report_deserialize_start_u32(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_u32");
    }

    fn report_deserialize_start_u64(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_u64");
    }

    fn report_deserialize_start_u128(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_u128");
    }

    fn report_deserialize_start_f32(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_f32");
    }

    fn report_deserialize_start_f64(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_f64");
    }

    fn report_deserialize_start_char(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_char");
    }

    fn report_deserialize_start_str(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_str");
    }

    fn report_deserialize_start_string(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_string");
    }

    fn report_deserialize_start_bytes(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_bytes");
    }

    fn report_deserialize_start_byte_buf(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_byte_buf");
    }

    fn report_deserialize_start_option(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_option");
    }

    fn report_deserialize_start_unit(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_unit");
    }

    fn report_deserialize_start_unit_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
    ) {
        trace!(
            nesting_level = self.level.get(),
            name,
            expecting = %args.expecting(), "start deserialize_unit_struct"
        );
    }

    fn report_deserialize_start_newtype_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
    ) {
        trace!(
            nesting_level = self.level.get(),
            name,
            expecting = %args.expecting(), "start deserialize_newtype_struct"
        );
    }

    fn report_deserialize_start_seq(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_seq");
    }

    fn report_deserialize_start_tuple(&mut self, args: impl DeserializeStartArgs, len: usize) {
        trace!(nesting_level = self.level.get(), len, expecting = %args.expecting(), "start deserialize_tuple");
    }

    fn report_deserialize_start_tuple_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
        len: usize,
    ) {
        trace!(
            nesting_level = self.level.get(),
            name,
            len,
            expecting = %args.expecting(), "start deserialize_tuple_struct"
        );
    }

    fn report_deserialize_start_map(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_map");
    }

    fn report_deserialize_start_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
        fields: &'static [&'static str],
    ) {
        trace!(
            nesting_level = self.level.get(),
            name,
            ?fields,
            expecting = %args.expecting(), "start deserialize_struct"
        );
    }

    fn report_deserialize_start_enum(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
        variants: &'static [&'static str],
    ) {
        trace!(
            nesting_level = self.level.get(),
            name,
            ?variants,
            expecting = %args.expecting(), "start deserialize_enum"
        );
    }

    fn report_deserialize_start_identifier(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_identifier");
    }

    fn report_deserialize_start_ignored_any(&mut self, args: impl DeserializeStartArgs) {
        trace!(nesting_level = self.level.get(), expecting = %args.expecting(), "start deserialize_ignored_any");
    }

    fn report_deserialize_finish(&mut self, error: Option<&dyn StdError>) {
        // We would like to log errors as tracing::Value, but that requires the error
        // type to be 'static. Which we can make it for our deserializer (in this
        // method), but not for our visitor (in the `report_recv_visit_*` methods).
        trace!(
            nesting_level = self.level.get(),
            error = error.map(tracing::field::display),
            "finish deserialize"
        );
    }

    fn report_deserialize_fallback_use_saved_value(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "deserializer failed, but we return Ok with value from the visitor"
        );
    }

    fn report_recv_visit_finish_primitive(&mut self, error: Option<&dyn StdError>) {
        trace!(
            nesting_level = self.level.get(),
            error = error.map(tracing::field::display),
            "finish receive visit_primitive"
        );
    }

    fn report_recv_visit_start_bool(&mut self, v: bool) {
        trace!(nesting_level = self.level.get(), v, "receive visit_bool");
    }

    fn report_recv_visit_start_i8(&mut self, v: i8) {
        trace!(nesting_level = self.level.get(), v, "receive visit_i8");
    }

    fn report_recv_visit_start_i16(&mut self, v: i16) {
        trace!(nesting_level = self.level.get(), v, "receive visit_i16");
    }

    fn report_recv_visit_start_i32(&mut self, v: i32) {
        trace!(nesting_level = self.level.get(), v, "receive visit_i32");
    }

    fn report_recv_visit_start_i64(&mut self, v: i64) {
        trace!(nesting_level = self.level.get(), v, "receive visit_i64");
    }

    fn report_recv_visit_start_i128(&mut self, v: i128) {
        trace!(nesting_level = self.level.get(), v, "receive visit_i128");
    }

    fn report_recv_visit_start_u8(&mut self, v: u8) {
        trace!(nesting_level = self.level.get(), v, "receive visit_u8");
    }

    fn report_recv_visit_start_u16(&mut self, v: u16) {
        trace!(nesting_level = self.level.get(), v, "receive visit_u16");
    }

    fn report_recv_visit_start_u32(&mut self, v: u32) {
        trace!(nesting_level = self.level.get(), v, "receive visit_u32");
    }

    fn report_recv_visit_start_u64(&mut self, v: u64) {
        trace!(nesting_level = self.level.get(), v, "receive visit_u64");
    }

    fn report_recv_visit_start_u128(&mut self, v: u128) {
        trace!(nesting_level = self.level.get(), v, "receive visit_u128");
    }

    fn report_recv_visit_start_f32(&mut self, v: f32) {
        trace!(nesting_level = self.level.get(), v, "receive visit_f32");
    }

    fn report_recv_visit_start_f64(&mut self, v: f64) {
        trace!(nesting_level = self.level.get(), v, "receive visit_f64");
    }

    fn report_recv_visit_start_char(&mut self, v: char) {
        trace!(nesting_level = self.level.get(), ?v, "receive visit_char");
    }

    fn report_recv_visit_start_str(&mut self, v: &str) {
        trace!(nesting_level = self.level.get(), v, "receive visit_str");
    }

    fn report_recv_visit_start_borrowed_str(&mut self, v: &str) {
        trace!(
            nesting_level = self.level.get(),
            v,
            "receive visit_borrowed_str"
        );
    }

    fn report_recv_visit_start_string(&mut self, v: &str) {
        trace!(nesting_level = self.level.get(), ?v, "receive visit_string");
    }

    fn report_recv_visit_start_bytes(&mut self, v: &[u8]) {
        trace!(nesting_level = self.level.get(), v, "receive visit_bytes");
    }

    fn report_recv_visit_start_borrowed_bytes(&mut self, v: &[u8]) {
        trace!(
            nesting_level = self.level.get(),
            v,
            "receive visit_borrowed_bytes"
        );
    }

    fn report_recv_visit_start_byte_buf(&mut self, v: &[u8]) {
        trace!(
            nesting_level = self.level.get(),
            ?v,
            "receive visit_byte_buf"
        );
    }

    fn report_recv_visit_start_none(&mut self) {
        trace!(nesting_level = self.level.get(), "receive visit_none");
    }

    fn report_recv_visit_start_some(&mut self) {
        trace!(nesting_level = self.level.get(), "receive visit_some");
        self.increase_level();
    }

    fn report_recv_visit_finish_some(&mut self, error: Option<&dyn StdError>) {
        self.decrease_level();
        trace!(
            nesting_level = self.level.get(),
            error = error.map(tracing::field::display),
            "finish receive visit_some"
        );
    }

    fn report_recv_visit_start_unit(&mut self) {
        trace!(nesting_level = self.level.get(), "receive visit_unit");
    }

    fn report_recv_visit_start_newtype_struct(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "receive visit_newtype_struct"
        );
        self.increase_level();
    }

    fn report_recv_visit_finish_newtype_struct(&mut self, error: Option<&dyn StdError>) {
        self.decrease_level();
        trace!(
            nesting_level = self.level.get(),
            error = error.map(tracing::field::display),
            "finish receive visit_newtype_struct"
        );
    }

    fn report_recv_visit_start_seq(&mut self) {
        trace!(nesting_level = self.level.get(), "receive visit_seq");
        self.increase_level();
    }

    fn report_recv_visit_finish_seq(&mut self, error: Option<&dyn StdError>) {
        self.decrease_level();
        trace!(
            nesting_level = self.level.get(),
            error = error.map(tracing::field::display),
            "finish receive visit_seq"
        );
    }

    fn report_recv_visit_start_map(&mut self) {
        trace!(nesting_level = self.level.get(), "receive visit_map");
        self.increase_level();
    }

    fn report_recv_visit_finish_map(&mut self, error: Option<&dyn StdError>) {
        self.decrease_level();
        trace!(
            nesting_level = self.level.get(),
            error = error.map(tracing::field::display),
            "finish receive visit_map"
        );
    }

    fn report_recv_visit_start_enum(&mut self) {
        trace!(nesting_level = self.level.get(), "receive visit_enum");
        self.increase_level();
    }

    fn report_recv_visit_finish_enum(&mut self, error: Option<&dyn StdError>) {
        self.decrease_level();
        trace!(
            nesting_level = self.level.get(),
            error = error.map(tracing::field::display),
            "finish receive visit_enum"
        );
    }

    fn report_new_halting_point(&mut self, point: &HaltingPoint) {
        trace!(nesting_level = self.level.get(), point = %point, "new halting point");
    }

    fn report_start_intervention(
        &mut self,
        reason: impl Debug,
        candidate_halting_point_for_next_attempt: Option<&HaltingPoint>,
        halting_point_stack: &[HaltingPoint],
    ) {
        trace!(
            nesting_level = self.level.get(),
            ?reason,
            candidate_halting_point_for_next_attempt =
                candidate_halting_point_for_next_attempt.map(tracing::field::display),
            // We have to format to a string to avoid a panic (#2).
            // Passing FormatIterator directly will type-check but panic in
            // some cases.
            halting_point_stack = format!("[{}]", FormatIterator::new(halting_point_stack.iter())),
            "start intervention"
        );
    }

    fn report_start_fallback(&mut self) {}

    fn report_no_fallback(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "no fallback was attempted"
        );
    }

    fn report_fallback(&mut self, error: Option<&dyn StdError>) {
        if let Some(error) = error {
            trace!(
                nesting_level = self.level.get(),
                error = %error,
                "fallback attempted but failed"
            );
        } else {
            trace!(nesting_level = self.level.get(), "fallback applied");
        }
    }

    fn report_fallback_no_element(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "fallback applied, no element"
        );
    }

    fn report_seq_next_element_start(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "start deserializing next element"
        );
    }

    fn report_seq_next_element_finish(&mut self, present: bool, error: Option<&dyn StdError>) {
        if let Some(error) = error {
            trace!(
                nesting_level = self.level.get(),
                error = %error,
                "finish deserializing next element"
            );
        } else {
            trace!(
                nesting_level = self.level.get(),
                present,
                "finish deserializing next element"
            );
        }
    }

    fn report_seq_next_element_skip(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "not deserializing next element"
        );
    }

    fn report_map_next_key_start(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "start deserializing next key"
        );
    }

    fn report_map_next_key_finish(&mut self, present: bool, error: Option<&dyn StdError>) {
        if let Some(error) = error {
            trace!(
                nesting_level = self.level.get(),
                error = %error,
                "finish deserializing next key"
            );
        } else {
            trace!(
                nesting_level = self.level.get(),
                present,
                "finish deserializing next key"
            );
        }
    }

    fn report_map_next_key_skip(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "not deserializing next key"
        );
    }

    fn report_map_next_value_start(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "start deserializing next value"
        );
    }

    fn report_map_next_value_finish(&mut self, error: Option<&dyn StdError>) {
        trace!(
            nesting_level = self.level.get(),
            error = error.map(tracing::field::display),
            "finish deserializing next value"
        );
    }

    fn report_access_past_end(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "access past end of collection"
        );
    }

    fn report_enum_start(&mut self) {
        trace!(nesting_level = self.level.get(), "start deserializing enum");
    }

    fn report_enum_finish(&mut self, error: Option<&dyn StdError>) {
        trace!(
            nesting_level = self.level.get(),
            error = error.map(tracing::field::display),
            "finish deserializing enum"
        );
    }

    fn report_variant_start_unit_variant(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "start deserializing unit variant"
        );
    }

    fn report_variant_start_newtype_variant(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "start deserializing newtype variant"
        );
    }

    fn report_variant_start_tuple_variant(&mut self, len: usize) {
        trace!(
            nesting_level = self.level.get(),
            len,
            "start deserializing tuple variant"
        );
    }

    fn report_variant_start_struct_variant(&mut self, fields: &'static [&'static str]) {
        trace!(
            nesting_level = self.level.get(),
            ?fields,
            "start deserializing struct variant"
        );
    }

    fn report_variant_finish(&mut self, error: Option<&dyn StdError>) {
        trace!(
            nesting_level = self.level.get(),
            error = error.map(tracing::field::display),
            "finish deserializing variant"
        );
    }

    fn report_allow_incomplete_string(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "encountered incomplete string"
        );
    }

    fn report_reject_incomplete_string(&mut self) {
        trace!(
            nesting_level = self.level.get(),
            "rejected incomplete string"
        );
    }
}

struct FormatIterator<It>(RefCell<Option<It>>);

impl<It> FormatIterator<It> {
    fn new(it: It) -> Self {
        Self(RefCell::new(Some(it)))
    }
}

impl<It> std::fmt::Display for FormatIterator<It>
where
    It: Iterator,
    It::Item: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iterator = self
            .0
            .borrow_mut()
            .take()
            .expect("used FormatIterator twice");
        f.write_str("[")?;
        for (i, item) in iterator.enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            item.fmt(f)?;
        }
        f.write_str("]")
    }
}
