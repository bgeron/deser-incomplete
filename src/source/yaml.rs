use super::Source;

/// Essentially [`serde_yaml::from_str`].
pub struct YamlStr<'de, T: std::borrow::Borrow<str> + ?Sized>(pub &'de T);

/// Essentially [`serde_yaml::from_slice`].
pub struct YamlBytes<'de, T: std::borrow::Borrow<[u8]> + ?Sized>(pub &'de T);

impl<'de, T: std::borrow::Borrow<str> + ?Sized> Source<'de> for YamlStr<'de, T> {
    type DeserializerStorage = serde_yaml::Deserializer<'de>;
    type Deserializer<'storage>
        = serde_yaml::Deserializer<'de>
    where
        'de: 'storage;
    type Error = serde_yaml::Error;

    fn recreate_deserializer_storage(&mut self) -> Self::DeserializerStorage {
        serde_yaml::Deserializer::from_str(self.0.borrow())
    }

    fn use_deserializer_from_storage<'storage>(
        storage: &mut Option<serde_yaml::Deserializer<'de>>,
    ) -> serde_yaml::Deserializer<'de> {
        storage
            .take()
            .expect("use_deserializer_from_storage only called on Some")
    }
}

impl<'de, T: std::borrow::Borrow<[u8]> + ?Sized> Source<'de> for YamlBytes<'de, T> {
    type DeserializerStorage = serde_yaml::Deserializer<'de>;
    type Deserializer<'storage>
        = serde_yaml::Deserializer<'de>
    where
        'de: 'storage;
    type Error = serde_yaml::Error;

    fn recreate_deserializer_storage(&mut self) -> Self::DeserializerStorage {
        serde_yaml::Deserializer::from_slice(self.0.borrow())
    }

    fn use_deserializer_from_storage<'storage>(
        storage: &mut Option<serde_yaml::Deserializer<'de>>,
    ) -> serde_yaml::Deserializer<'de> {
        storage
            .take()
            .expect("use_deserializer_from_storage only called on Some")
    }
}
