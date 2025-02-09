use crate::attempt::HaltingPoint;
#[cfg(doc)]
use crate::fallback::Fallbacks;
use crate::util::DeserializeKind;
use std::error::Error as StdError;
use std::fmt::{Debug, Formatter};

use serde::de::Visitor;
#[cfg(doc)]
use serde::de::{MapAccess, SeqAccess};

mod default_reporter;

pub use default_reporter::DefaultReporter;

/// Reporters are for inspecting the normal behavior of this library.
///
/// Logic errors are typically not exposed here, and will be logged with `tracing`
/// instead.
pub trait Reporter: Clone {
    fn report_deserialize_start_any(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_bool(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_i8(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_i16(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_i32(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_i64(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_i128(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_u8(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_u16(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_u32(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_u64(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_u128(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_f32(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_f64(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_char(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_str(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_string(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_bytes(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_byte_buf(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_option(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_unit(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_unit_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
    );
    fn report_deserialize_start_newtype_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
    );
    fn report_deserialize_start_seq(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_tuple(&mut self, args: impl DeserializeStartArgs, len: usize);
    fn report_deserialize_start_tuple_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
        len: usize,
    );
    fn report_deserialize_start_map(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_struct(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
        fields: &'static [&'static str],
    );
    fn report_deserialize_start_enum(
        &mut self,
        args: impl DeserializeStartArgs,
        name: &'static str,
        variants: &'static [&'static str],
    );
    fn report_deserialize_start_identifier(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_start_ignored_any(&mut self, args: impl DeserializeStartArgs);
    fn report_deserialize_finish(&mut self, error: Option<&dyn StdError>);
    fn report_deserialize_fallback_use_saved_value(&mut self);

    /// This is called after visiting anything that doesn't have its own
    /// `report_end_*` method.
    fn report_recv_visit_finish_primitive(&mut self, error: Option<&dyn StdError>);
    fn report_recv_visit_start_bool(&mut self, v: bool);
    fn report_recv_visit_start_i8(&mut self, v: i8);
    fn report_recv_visit_start_i16(&mut self, v: i16);
    fn report_recv_visit_start_i32(&mut self, v: i32);
    fn report_recv_visit_start_i64(&mut self, v: i64);
    fn report_recv_visit_start_i128(&mut self, v: i128);
    fn report_recv_visit_start_u8(&mut self, v: u8);
    fn report_recv_visit_start_u16(&mut self, v: u16);
    fn report_recv_visit_start_u32(&mut self, v: u32);
    fn report_recv_visit_start_u64(&mut self, v: u64);
    fn report_recv_visit_start_u128(&mut self, v: u128);
    fn report_recv_visit_start_f32(&mut self, v: f32);
    fn report_recv_visit_start_f64(&mut self, v: f64);
    fn report_recv_visit_start_char(&mut self, v: char);
    fn report_recv_visit_start_str(&mut self, v: &str);
    fn report_recv_visit_start_borrowed_str(&mut self, v: &str);
    fn report_recv_visit_start_string(&mut self, v: &str);
    fn report_recv_visit_start_bytes(&mut self, v: &[u8]);
    fn report_recv_visit_start_borrowed_bytes(&mut self, v: &[u8]);
    fn report_recv_visit_start_byte_buf(&mut self, v: &[u8]);
    fn report_recv_visit_start_none(&mut self);
    fn report_recv_visit_start_some(&mut self);
    fn report_recv_visit_finish_some(&mut self, error: Option<&dyn StdError>);
    fn report_recv_visit_start_unit(&mut self);
    fn report_recv_visit_start_newtype_struct(&mut self);
    fn report_recv_visit_finish_newtype_struct(&mut self, error: Option<&dyn StdError>);
    fn report_recv_visit_start_seq(&mut self);
    fn report_recv_visit_finish_seq(&mut self, error: Option<&dyn StdError>);
    fn report_recv_visit_start_map(&mut self);
    fn report_recv_visit_finish_map(&mut self, error: Option<&dyn StdError>);
    fn report_recv_visit_start_enum(&mut self);
    fn report_recv_visit_finish_enum(&mut self, error: Option<&dyn StdError>);

    fn report_new_halting_point(&mut self, point: &HaltingPoint);
    fn report_start_intervention(
        &mut self,
        reason: impl Debug,
        candidate_halting_point_for_next_attempt: Option<&HaltingPoint>,
        halting_point_stack: &[HaltingPoint],
    );

    /// The deserializer failed without consuming the visitor. We start computing one of the [`Fallbacks`].
    fn report_start_fallback(&mut self);

    /// The deserializer failed without consuming the visitor, but we chose not to apply
    /// one of the [`Fallbacks`]. (This is reported when the fallback does not take the visitor.)
    fn report_no_fallback(&mut self);

    /// The deserializer failed without consuming the visitor, and one of the [`Fallbacks`] was applied,
    /// or at least attempted.
    fn report_fallback(&mut self, error: Option<&dyn StdError>);

    /// From an Access type, we return as a fallback that there is no element left.
    fn report_fallback_no_element(&mut self);

    fn report_seq_next_element_start(&mut self);
    fn report_seq_next_element_finish(&mut self, present: bool, error: Option<&dyn StdError>);
    /// The next element was requested from [`SeqAccess`], but we're not checking
    /// if there is a next element, we just decide it it's not going to be there.
    ///
    /// This could be for instance because we encountered our halting point.
    fn report_seq_next_element_skip(&mut self);

    fn report_map_next_key_start(&mut self);
    fn report_map_next_key_finish(&mut self, present: bool, error: Option<&dyn StdError>);
    /// The next key was requested from [`MapAccess`], but we're not checking
    /// if there is a next field, we just decide it it's not going to be there.
    ///
    /// This could be for instance because we encountered our halting point.
    fn report_map_next_key_skip(&mut self);
    fn report_map_next_value_start(&mut self);
    fn report_map_next_value_finish(&mut self, error: Option<&dyn StdError>);

    /// The data type attempted to read a value from a collection after we already
    /// reported that there are no more.
    fn report_access_past_end(&mut self);

    fn report_enum_start(&mut self);
    fn report_enum_finish(&mut self, error: Option<&dyn StdError>);
    fn report_variant_start_unit_variant(&mut self);
    fn report_variant_start_newtype_variant(&mut self);
    fn report_variant_start_tuple_variant(&mut self, len: usize);
    fn report_variant_start_struct_variant(&mut self, fields: &'static [&'static str]);
    fn report_variant_finish(&mut self, error: Option<&dyn StdError>);

    fn report_allow_incomplete_string(&mut self);
    fn report_reject_incomplete_string(&mut self);
}

pub trait DeserializeStartArgs {
    fn expecting_fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result;
}

/// Not public interface in the foreseeable future.
pub(crate) trait DeserializeStartArgsExt: DeserializeStartArgs {
    // Convenience wrapper around `expecting_fmt`
    fn expecting(&self) -> impl std::fmt::Display + '_ {
        Expecting(self)
    }
}

impl<T> DeserializeStartArgsExt for T where T: DeserializeStartArgs {}

pub struct Expecting<'a, T: ?Sized>(&'a T);

impl<T> std::fmt::Display for Expecting<'_, T>
where
    T: DeserializeStartArgs + ?Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.expecting_fmt(f)
    }
}

pub(crate) struct DeserializeStartArgsImpl<'a, V: 'a> {
    pub(crate) visitor: &'a V,
}

impl<'de, V> DeserializeStartArgs for DeserializeStartArgsImpl<'_, V>
where
    V: Visitor<'de>,
{
    fn expecting_fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.visitor.expecting(formatter)
    }
}

impl<T> ReporterExt for T where T: Reporter {}

pub(crate) trait ReporterExt: Reporter {
    /// Not public interface in the foreseeable future.
    fn report_deserialize_start(&mut self, args: impl DeserializeStartArgs, kind: DeserializeKind) {
        match kind {
            DeserializeKind::Any => self.report_deserialize_start_any(args),
            DeserializeKind::Bool => self.report_deserialize_start_bool(args),
            DeserializeKind::I8 => self.report_deserialize_start_i8(args),
            DeserializeKind::I16 => self.report_deserialize_start_i16(args),
            DeserializeKind::I32 => self.report_deserialize_start_i32(args),
            DeserializeKind::I64 => self.report_deserialize_start_i64(args),
            DeserializeKind::I128 => self.report_deserialize_start_i128(args),
            DeserializeKind::U8 => self.report_deserialize_start_u8(args),
            DeserializeKind::U16 => self.report_deserialize_start_u16(args),
            DeserializeKind::U32 => self.report_deserialize_start_u32(args),
            DeserializeKind::U64 => self.report_deserialize_start_u64(args),
            DeserializeKind::U128 => self.report_deserialize_start_u128(args),
            DeserializeKind::F32 => self.report_deserialize_start_f32(args),
            DeserializeKind::F64 => self.report_deserialize_start_f64(args),
            DeserializeKind::Char => self.report_deserialize_start_char(args),
            DeserializeKind::Str => self.report_deserialize_start_str(args),
            DeserializeKind::String => self.report_deserialize_start_string(args),
            DeserializeKind::Bytes => self.report_deserialize_start_bytes(args),
            DeserializeKind::ByteBuf => self.report_deserialize_start_byte_buf(args),
            DeserializeKind::Option => self.report_deserialize_start_option(args),
            DeserializeKind::Unit => self.report_deserialize_start_unit(args),
            DeserializeKind::UnitStruct { name } => {
                self.report_deserialize_start_unit_struct(args, name)
            }
            DeserializeKind::NewtypeStruct { name } => {
                self.report_deserialize_start_newtype_struct(args, name)
            }
            DeserializeKind::Seq => self.report_deserialize_start_seq(args),
            DeserializeKind::Tuple { len } => self.report_deserialize_start_tuple(args, len),
            DeserializeKind::TupleStruct { name, len } => {
                self.report_deserialize_start_tuple_struct(args, name, len)
            }
            DeserializeKind::Map => self.report_deserialize_start_map(args),
            DeserializeKind::Struct { name, fields } => {
                self.report_deserialize_start_struct(args, name, fields)
            }
            DeserializeKind::Enum { name, variants } => {
                self.report_deserialize_start_enum(args, name, variants)
            }
            DeserializeKind::Identifier => self.report_deserialize_start_identifier(args),
            DeserializeKind::IgnoredAny => self.report_deserialize_start_ignored_any(args),
        }
    }
}
