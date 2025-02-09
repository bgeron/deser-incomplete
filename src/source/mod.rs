//! Trait [`Source`] describes a resource that can be deserialized multiple times:
//! a [`Deserializer`] together with the data it works on, such as a string
//! reference or byte slice.

use serde::Deserializer;

#[cfg(feature = "serde_json")]
mod json;
#[cfg(feature = "serde_yaml")]
mod yaml;

#[cfg(feature = "serde_json")]
pub use json::{JsonBytes, JsonStr};
#[cfg(feature = "serde_yaml")]
pub use yaml::{YamlBytes, YamlStr};

/// Describes a resource that can be deserialized multiple times:
/// a [`Deserializer`] together with the data it works on, such as a string
/// reference or byte slice.
///
/// For instance, [`crate::source::JsonStr`] is a borrowed string that we should use [`serde_json`] on.
///
/// For your own data format, take a look at the implementation of either [`JsonStr`]
/// and [`YamlStr`] --- depending on whether your data format implements [`Deserializer`]
/// like [`serde_json`] or like [`serde_yaml`]:
///
/// - [`serde_yaml`] has `serde_yaml::Deserializer: serde::Deserializer`
/// - [`serde_json`] has `&mut serde_json::Deserializer: serde::Deserializer`
pub trait Source<'de> {
    /// Stack storage for the deserializer.
    type DeserializerStorage;

    type Deserializer<'storage>: Deserializer<'de, Error = Self::Error>
    where
        'de: 'storage;

    /// The error type cannot depend on `'storage`.
    type Error: serde::de::Error;

    /// Recreate a deserializer for this source.
    ///
    /// Every deserializer created from a source should behave exactly the
    /// same.
    ///
    /// If end of file happens in a map in between the key and the value, then
    /// our first go at deserializing will fail, and we have to recreate
    /// a new deserializer for the same source.
    fn recreate_deserializer_storage(&mut self) -> Self::DeserializerStorage;

    /// Will be called exactly once per [`Self::DeserializerStorage`]. The argument
    /// is guaranteed to be `Some`.
    ///
    /// Typically returns either `storage.take()` or `&mut storage`.
    fn use_deserializer_from_storage(
        storage: &mut Option<Self::DeserializerStorage>,
    ) -> Self::Deserializer<'_>;
}
