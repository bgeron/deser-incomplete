use std::error::Error as StdError;

use super::access::Access;
use super::{erase_error_ref, Deserializer};
use crate::options_impl::ExtraOptions;
use crate::reporter::Reporter;
use crate::state::{AttemptState, GlobalState};
use crate::util::DeserializeKind;
use crate::Error;

/// Something that creates a data value, if only you tell it what the format is like.
pub(crate) struct Visitor<'a, 'de, Inner, Extra>
where
    Inner: serde::de::Visitor<'de>,
    Extra: ExtraOptions,
{
    pub(super) global: &'a mut GlobalState<Extra>,
    pub(super) attempt: &'a mut AttemptState<Extra>,
    pub(super) kind: DeserializeKind,
    pub(super) is_at_root: bool,
    pub(super) is_for_key_or_variant: bool,

    /// This should always be set to `Some` while the inner deserializer is being called,
    /// and thus while the [`serde::de::Visitor`] methods of [`Visitor`] are called.
    ///
    /// The inner visitor actually lives on the stack, so that in case the deserializer fails,
    /// we can attempt to apply a fallback instead.
    pub(super) inner: &'a mut Option<Inner>,

    /// If the inner visitor returns a value, we store it here for safekeeping
    /// so we can recover if the deserializer decides to error anyway.
    pub(super) value: &'a mut Option<Inner::Value>,
}

fn framework<'de, Inner, Extra, E>(
    visitor: Visitor<'_, 'de, Inner, Extra>,
    do_visit: impl FnOnce(
        Inner,
        (
            &mut GlobalState<Extra>,
            &mut AttemptState<Extra>,
            DeserializeKind,
        ),
    ) -> Result<Inner::Value, E>,
    report_end: impl FnOnce(&mut Extra::Reporter, Option<&dyn StdError>),
) -> Result<(), E>
where
    Inner: serde::de::Visitor<'de>,
    Extra: ExtraOptions,
    E: serde::de::Error,
{
    let inner_visitor = visitor
        .inner
        .take()
        .expect("inner visitor is present when running Visitor");

    let result = do_visit(
        inner_visitor,
        (visitor.global, visitor.attempt, visitor.kind),
    );

    if result.is_err() {
        visitor
            .attempt
            .activate_intervention(crate::state::InterventionReason::VisitError);
    }
    report_end(&mut visitor.global.reporter, erase_error_ref(&result));

    match result {
        Ok(value) => {
            *visitor.value = Some(value);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

impl<'de, Inner, Extra> serde::de::Visitor<'de> for Visitor<'_, 'de, Inner, Extra>
where
    Inner: serde::de::Visitor<'de>,
    Extra: ExtraOptions,
{
    /// The visitor actually does not return the value, but stores it in a higher stack frame.
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner
            .as_ref()
            .expect("the inner visitor has not been consumed while the external deserializer is running")
            .expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_bool(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_bool(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_i8(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_i8(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_i16(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_i16(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_i32(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_i32(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_i64(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_i64(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_i128(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_i128(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_u8(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_u8(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_u16(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_u16(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_u32(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_u32(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_u64(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_u64(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_u128(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_u128(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_f32(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_f32(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_f64(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_f64(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_char(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_char(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_str<E>(self, mut v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.global.config.remove_tag_from_stringlike(&mut v) {
            if !self.is_for_key_or_variant
                || self
                    .global
                    .config
                    .behavior
                    .allow_incomplete_string_in_key_or_variant
            {
                self.global.reporter.report_allow_incomplete_string();
            } else {
                self.global.reporter.report_reject_incomplete_string();
                return Err(E::custom(
                    "not allowing incomplete string in key or variant",
                ));
            }
        }

        self.global.reporter.report_recv_visit_start_str(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_str(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_borrowed_str<E>(self, mut v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.global.config.remove_tag_from_stringlike(&mut v) {
            if !self.is_for_key_or_variant
                || self
                    .global
                    .config
                    .behavior
                    .allow_incomplete_string_in_key_or_variant
            {
                self.global.reporter.report_allow_incomplete_string();
            } else {
                self.global.reporter.report_reject_incomplete_string();
                return Err(E::custom(
                    "not allowing incomplete string in key or variant",
                ));
            }
        }

        self.global.reporter.report_recv_visit_start_borrowed_str(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_borrowed_str(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_string<E>(self, mut v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.global.config.remove_tag_from_stringlike(&mut v) {
            if !self.is_for_key_or_variant
                || self
                    .global
                    .config
                    .behavior
                    .allow_incomplete_string_in_key_or_variant
            {
                self.global.reporter.report_allow_incomplete_string();
            } else {
                self.global.reporter.report_reject_incomplete_string();
                return Err(E::custom(
                    "not allowing incomplete string in key or variant",
                ));
            }
        }

        self.global.reporter.report_recv_visit_start_string(&v);

        framework(
            self,
            |visitor, _extra| visitor.visit_string(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_bytes<E>(self, mut v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.global.config.remove_tag_from_stringlike(&mut v) {
            if !self.is_for_key_or_variant
                || self
                    .global
                    .config
                    .behavior
                    .allow_incomplete_string_in_key_or_variant
            {
                self.global.reporter.report_allow_incomplete_string();
            } else {
                self.global.reporter.report_reject_incomplete_string();
                return Err(E::custom(
                    "not allowing incomplete string in key or variant",
                ));
            }
        }

        self.global.reporter.report_recv_visit_start_bytes(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_bytes(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_borrowed_bytes<E>(self, mut v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.global.config.remove_tag_from_stringlike(&mut v) {
            if !self.is_for_key_or_variant
                || self
                    .global
                    .config
                    .behavior
                    .allow_incomplete_string_in_key_or_variant
            {
                self.global.reporter.report_allow_incomplete_string();
            } else {
                self.global.reporter.report_reject_incomplete_string();
                return Err(E::custom(
                    "not allowing incomplete string in key or variant",
                ));
            }
        }

        self.global
            .reporter
            .report_recv_visit_start_borrowed_bytes(v);

        framework(
            self,
            |visitor, _extra| visitor.visit_borrowed_bytes(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_byte_buf<E>(self, mut v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if self.global.config.remove_tag_from_stringlike(&mut v) {
            if !self.is_for_key_or_variant
                || self
                    .global
                    .config
                    .behavior
                    .allow_incomplete_string_in_key_or_variant
            {
                self.global.reporter.report_allow_incomplete_string();
            } else {
                self.global.reporter.report_reject_incomplete_string();
                return Err(E::custom(
                    "not allowing incomplete string in key or variant",
                ));
            }
        }

        self.global.reporter.report_recv_visit_start_byte_buf(&v);

        framework(
            self,
            |visitor, _extra| visitor.visit_byte_buf(v),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_none();

        framework(
            self,
            |visitor, _extra| visitor.visit_none(),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.global.reporter.report_recv_visit_start_some();
        let is_at_root = self.is_at_root;

        framework(
            self,
            |visitor, (global, attempt, _kind)| {
                let wrapped = Deserializer {
                    global,
                    attempt,
                    is_at_root,
                    is_for_key_or_variant: false,
                    is_for_map_value: false,
                    inner: deserializer,
                };
                visitor
                    .visit_some(wrapped)
                    .map_err(Error::unpack_or_make_custom)
            },
            |reporter, error| {
                reporter.report_recv_visit_finish_some(error);
            },
        )
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.global.reporter.report_recv_visit_start_unit();

        framework(
            self,
            |visitor, _extra| visitor.visit_unit(),
            |reporter, error| {
                reporter.report_recv_visit_finish_primitive(error);
            },
        )
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.global
            .reporter
            .report_recv_visit_start_newtype_struct();
        let is_at_root = self.is_at_root;

        framework(
            self,
            |visitor, (global, attempt, _kind)| {
                let wrapped = Deserializer {
                    global,
                    attempt,
                    is_at_root,
                    is_for_key_or_variant: false,
                    is_for_map_value: false,
                    inner: deserializer,
                };
                visitor
                    .visit_newtype_struct(wrapped)
                    .map_err(Error::unpack_or_make_custom)
            },
            |reporter, error| {
                reporter.report_recv_visit_finish_newtype_struct(error);
            },
        )
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        self.global.reporter.report_recv_visit_start_seq();

        framework(
            self,
            |visitor, (global, attempt, kind)| {
                visitor.visit_seq(Access {
                    global,
                    attempt,
                    kind,
                    inner: seq,
                    collection_has_ended: false,
                    inside_element: None,
                })
            },
            |reporter, error| {
                reporter.report_recv_visit_finish_seq(error);
            },
        )
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        self.global.reporter.report_recv_visit_start_map();

        framework(
            self,
            |visitor, (global, attempt, kind)| {
                visitor.visit_map(Access {
                    global,
                    attempt,
                    kind,
                    inner: map,
                    collection_has_ended: false,
                    inside_element: None,
                })
            },
            |reporter, error| {
                reporter.report_recv_visit_finish_map(error);
            },
        )
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        self.global.reporter.report_recv_visit_start_enum();

        framework(
            self,
            |visitor, (global, attempt, kind)| {
                visitor.visit_enum(Access {
                    global,
                    attempt,
                    kind,
                    inner: data,
                    collection_has_ended: false,
                    inside_element: None,
                })
            },
            |reporter, error| {
                reporter.report_recv_visit_finish_enum(error);
            },
        )
    }
}
