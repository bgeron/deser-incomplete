use serde::de::Visitor;

use crate::attempt::empty_access::EmptyAccess;
use crate::util::DeserializeKind;
use crate::UnstableCustomBehavior;

#[derive(Debug)]
pub struct FallbackContext {
    pub(crate) is_at_root: bool,
    pub(crate) is_for_map_value: bool,
}

impl FallbackContext {
    pub fn is_at_root(&self) -> bool {
        self.is_at_root
    }

    pub fn is_for_map_value(&self) -> bool {
        self.is_for_map_value
    }

    pub fn is_at_mandatory(&self) -> bool {
        self.is_at_root() || self.is_for_map_value()
    }
}

/// This describes what to do in case the data ends unexpectedly.
///
/// For instance, for a data type
///
/// ```rs
/// # use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct X {
///     i: Option<i32>,
/// }
/// ```
///
/// for JSON input `{"i": `, at end-of-input we can pretend that the JSON
/// was `{"i": null}`.
///
/// This trait specifies one case for each method of [`serde::Deserializer`].
///
/// ## Context
///
/// Fallbacks aren't crucial to the functioning of this crate.
///
/// If there is no fallback (in the example above, if [`Self::fallback_option`]
/// returns `None`), then one round of deserialization will fail but we will
/// restart and next time we know to not attempt to serialize `"i"`.`
pub trait Fallbacks {
    fn fallback_any<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_bool<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_i8<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_i16<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_i32<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_i64<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_i128<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_u8<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_u16<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_u32<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_u64<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_u128<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_f32<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_f64<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_char<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_str<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_string<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_bytes<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_byte_buf<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_option<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_unit<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_unit_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_newtype_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_seq<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_tuple<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_tuple_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_map<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_enum<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_identifier<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
    fn fallback_ignored_any<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        _take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        Ok(None)
    }
}

pub struct DefaultFallbacks {
    pub(crate) behavior: UnstableCustomBehavior,
}

impl Fallbacks for DefaultFallbacks {
    fn fallback_str<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        if let Some(s) = self.behavior.fallback_default_str {
            (take_visitor)().visit_str(s).map(Some)
        } else {
            Ok(None)
        }
    }

    fn fallback_string<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        if let Some(s) = self.behavior.fallback_default_str {
            (take_visitor)().visit_string(s.to_string()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn fallback_option<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        conditional_fallback(
            if context.is_at_mandatory() {
                self.behavior.fallback_none_at_mandatory
            } else {
                self.behavior.fallback_none
            },
            move || (take_visitor)().visit_none(),
        )
    }

    fn fallback_unit<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        conditional_fallback(
            if context.is_at_mandatory() {
                self.behavior.fallback_unit_at_mandatory
            } else {
                self.behavior.fallback_unit
            },
            move || (take_visitor)().visit_unit(),
        )
    }

    fn fallback_unit_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        conditional_fallback(
            if context.is_at_mandatory() {
                self.behavior.fallback_unit_struct_at_mandatory
            } else {
                self.behavior.fallback_unit_struct
            },
            move || (take_visitor)().visit_unit(),
        )
    }
    fn fallback_seq<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        conditional_fallback(
            if context.is_at_root() {
                self.behavior.fallback_seq_empty_at_root
            } else {
                self.behavior.fallback_seq_empty
            },
            move || (take_visitor)().visit_seq(EmptyAccess::default()),
        )
    }
    fn fallback_map<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        conditional_fallback(
            if context.is_at_root() {
                self.behavior.fallback_map_empty_at_root
            } else {
                self.behavior.fallback_map_empty
            },
            move || (take_visitor)().visit_map(EmptyAccess::default()),
        )
    }
    fn fallback_struct<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        conditional_fallback(
            if context.is_at_root() {
                self.behavior.fallback_struct_empty_at_root
            } else {
                self.behavior.fallback_struct_empty
            },
            move || (take_visitor)().visit_map(EmptyAccess::default()),
        )
    }

    fn fallback_any<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        conditional_fallback(self.behavior.fallback_any_as_none, move || {
            (take_visitor)().visit_none()
        })
    }

    fn fallback_ignored_any<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        _context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
    ) -> Result<Option<V::Value>, E> {
        conditional_fallback(self.behavior.fallback_ignored_any_as_none, move || {
            (take_visitor)().visit_none()
        })
    }
}

fn conditional_fallback<Value, E>(
    test: bool,
    f: impl FnOnce() -> Result<Value, E>,
) -> Result<Option<Value>, E> {
    if test {
        f().map(Some)
    } else {
        Ok(None)
    }
}

impl<T> FallbacksExt for T where T: Fallbacks {}

/// Not public interface in the foreseeable future.
pub(crate) trait FallbacksExt: Fallbacks {
    fn fallback<'a, V: Visitor<'a>, E: serde::de::Error>(
        &self,
        context: &FallbackContext,
        take_visitor: impl FnOnce() -> V,
        kind: DeserializeKind,
    ) -> Result<Option<V::Value>, E> {
        match kind {
            DeserializeKind::Any => self.fallback_any(context, take_visitor),
            DeserializeKind::Bool => self.fallback_bool(context, take_visitor),
            DeserializeKind::I8 => self.fallback_i8(context, take_visitor),
            DeserializeKind::I16 => self.fallback_i16(context, take_visitor),
            DeserializeKind::I32 => self.fallback_i32(context, take_visitor),
            DeserializeKind::I64 => self.fallback_i64(context, take_visitor),
            DeserializeKind::I128 => self.fallback_i128(context, take_visitor),
            DeserializeKind::U8 => self.fallback_u8(context, take_visitor),
            DeserializeKind::U16 => self.fallback_u16(context, take_visitor),
            DeserializeKind::U32 => self.fallback_u32(context, take_visitor),
            DeserializeKind::U64 => self.fallback_u64(context, take_visitor),
            DeserializeKind::U128 => self.fallback_u128(context, take_visitor),
            DeserializeKind::F32 => self.fallback_f32(context, take_visitor),
            DeserializeKind::F64 => self.fallback_f64(context, take_visitor),
            DeserializeKind::Char => self.fallback_char(context, take_visitor),
            DeserializeKind::Str => self.fallback_str(context, take_visitor),
            DeserializeKind::String => self.fallback_string(context, take_visitor),
            DeserializeKind::Bytes => self.fallback_bytes(context, take_visitor),
            DeserializeKind::ByteBuf => self.fallback_byte_buf(context, take_visitor),
            DeserializeKind::Option => self.fallback_option(context, take_visitor),
            DeserializeKind::Unit => self.fallback_unit(context, take_visitor),
            DeserializeKind::UnitStruct { .. } => self.fallback_unit_struct(context, take_visitor),
            DeserializeKind::NewtypeStruct { .. } => {
                self.fallback_newtype_struct(context, take_visitor)
            }
            DeserializeKind::Seq => self.fallback_seq(context, take_visitor),
            DeserializeKind::Tuple { .. } => self.fallback_tuple(context, take_visitor),
            DeserializeKind::TupleStruct { .. } => {
                self.fallback_tuple_struct(context, take_visitor)
            }
            DeserializeKind::Map => self.fallback_map(context, take_visitor),
            DeserializeKind::Struct { .. } => self.fallback_struct(context, take_visitor),
            DeserializeKind::Enum { .. } => self.fallback_enum(context, take_visitor),
            DeserializeKind::Identifier => self.fallback_identifier(context, take_visitor),
            DeserializeKind::IgnoredAny => self.fallback_ignored_any(context, take_visitor),
        }
    }
}
