use std::marker::PhantomData;

/// Represents an empty sequence or map.
#[derive(Debug)]
pub(crate) struct EmptyAccess<E>(pub PhantomData<E>);

impl<E> Default for EmptyAccess<E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'a, E: serde::de::Error> serde::de::SeqAccess<'a> for EmptyAccess<E> {
    type Error = E;

    fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'a>,
    {
        Ok(None)
    }
}

impl<'a, E: serde::de::Error> serde::de::MapAccess<'a> for EmptyAccess<E> {
    type Error = E;

    fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'a>,
    {
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'a>,
    {
        Err(serde::de::Error::custom(
            "called next_value_seed before key",
        ))
    }
}
