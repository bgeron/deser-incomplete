mod access;
mod deserializer;
pub(crate) mod empty_access;
mod visit;

use std::fmt::Display;
use std::ops::Deref;

use crate::options_impl::ExtraOptions;
use crate::state::{AttemptState, GlobalState};
use crate::util::erase_error_ref;
use crate::Error;

/// Represents a point in the deserialization process where we could choose to stop
/// deserializing and save this attempt. For instance, before a map key or before a
/// sequence element.

// This is copyable in practice, but that's not something we want on the
// public interface.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HaltingPoint(pub(crate) u64);

impl HaltingPoint {
    pub(crate) fn increment(&mut self) {
        self.0 += 1;
    }
}

impl From<u64> for HaltingPoint {
    fn from(point: u64) -> Self {
        Self(point)
    }
}

impl Display for HaltingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "halting point {}", self.0)
    }
}

impl Deref for HaltingPoint {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// This is the deserializer with all options, including unstable interfaces.
pub(crate) struct Deserializer<'a, Inner, Extra>
where
    Extra: ExtraOptions,
{
    pub(crate) global: &'a mut GlobalState<Extra>,
    pub(crate) attempt: &'a mut AttemptState<Extra>,
    /// Whether we are at the root of the deserialization. Typically we will
    /// be a little bit more lenient at the root -- because we definitely do
    /// want to generate a value, and there isn't the risk that unparseable
    /// syntax will generate an unexpected spurious element that was never
    /// in the input.
    pub(crate) is_at_root: bool,
    /// Whether this is used to deserialize a map key or enum variant.
    pub(crate) is_for_key_or_variant: bool,
    /// Whether this is used to deserialize a map value or the value inside an
    /// enum -- somewhere we know for sure that a value should go there.
    pub(crate) is_for_map_value: bool,
    pub(crate) inner: Inner,
}

pub(crate) struct InnerDeserializeSeed<'a, Inner, Extra>
where
    Extra: ExtraOptions,
{
    pub(crate) global: &'a mut GlobalState<Extra>,
    pub(crate) attempt: &'a mut AttemptState<Extra>,
    pub(crate) is_for_key_or_variant: bool,
    pub(crate) is_for_map_value: bool,
    pub(crate) inner: Inner,
}

impl<'de, Inner, Extra> serde::de::DeserializeSeed<'de> for InnerDeserializeSeed<'_, Inner, Extra>
where
    Inner: serde::de::DeserializeSeed<'de>,
    Extra: ExtraOptions,
{
    type Value = Inner::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.inner
            .deserialize(Deserializer {
                global: self.global,
                attempt: self.attempt,
                is_at_root: false,
                is_for_key_or_variant: self.is_for_key_or_variant,
                is_for_map_value: self.is_for_map_value,
                inner: deserializer,
            })
            .map_err(Error::unpack_or_make_custom)
    }
}
