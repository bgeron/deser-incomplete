use super::Source;

/// Essentially [`serde_json::from_str`].
pub struct JsonStr<'de, T: std::borrow::Borrow<str> + ?Sized>(pub &'de T);

/// Essentially [`serde_json::from_slice`].
pub struct JsonBytes<'de, T: std::borrow::Borrow<[u8]> + ?Sized>(pub &'de T);

impl<'de, T: std::borrow::Borrow<str> + ?Sized> Source<'de> for JsonStr<'de, T> {
    type DeserializerStorage = serde_json::Deserializer<serde_json::de::StrRead<'de>>;
    type Deserializer<'storage>
        = &'storage mut serde_json::Deserializer<serde_json::de::StrRead<'de>>
    where
        'de: 'storage;
    type Error = serde_json::Error;

    fn recreate_deserializer_storage(&mut self) -> Self::DeserializerStorage {
        serde_json::Deserializer::from_str(self.0.borrow())
    }

    fn use_deserializer_from_storage<'storage>(
        storage: &'storage mut Option<serde_json::Deserializer<serde_json::de::StrRead<'de>>>,
    ) -> &'storage mut serde_json::Deserializer<serde_json::de::StrRead<'de>> {
        storage
            .as_mut()
            .expect("use_deserializer_from_storage only called on Some")
    }
}

impl<'de, T: std::borrow::Borrow<[u8]> + ?Sized> Source<'de> for JsonBytes<'de, T> {
    type DeserializerStorage = serde_json::Deserializer<serde_json::de::SliceRead<'de>>;
    type Deserializer<'storage>
        = &'storage mut serde_json::Deserializer<serde_json::de::SliceRead<'de>>
    where
        'de: 'storage;
    type Error = serde_json::Error;

    fn recreate_deserializer_storage(&mut self) -> Self::DeserializerStorage {
        serde_json::Deserializer::from_slice(self.0.borrow())
    }

    fn use_deserializer_from_storage<'storage>(
        storage: &'storage mut Option<serde_json::Deserializer<serde_json::de::SliceRead<'de>>>,
    ) -> &'storage mut serde_json::Deserializer<serde_json::de::SliceRead<'de>> {
        storage
            .as_mut()
            .expect("use_deserializer_from_storage only called on Some")
    }
}
