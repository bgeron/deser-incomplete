#![allow(clippy::len_without_is_empty)]

#[cfg(feature = "serde_json")]
pub(crate) mod json;
#[cfg(feature = "serde_yaml")]
pub(crate) mod yaml;

/// For some data formats, we like to append some randomized trailer
/// to the input before deserializing.
///
/// - For JSON, this lets us support incomplete strings.
///
/// - For YAML, we need this otherwise any incomplete string seems to
///   cause deserialization to fail.
///
/// However, such a trailer then needs to be removed from any decoded
/// strings.
pub trait RandomTrailer {
    /// Given a random tag, add the corresponding trailer to the input.
    fn prepare_string_with_tag(&self, input: &mut String, tag: &str);

    /// Given a random tag, add the corresponding trailer to the input.
    fn prepare_vec_with_tag(&self, input: &mut Vec<u8>, tag: &str);

    /// Given a parsed string or bytes, detect whether the random tag was
    /// parsed,
    /// then take off the suffix of the parsed string that may not have been
    /// present in the original input.
    ///
    /// Return whether the tag was detected. This simultaneously represents that
    /// we have encountered the end of the input, and we should probably stop
    /// trying to deserialize more.
    ///
    /// This method should only modify the input if the tag is detected.
    ///
    /// This method will not be called when parsing was done without a random tag.
    #[must_use]
    fn remove_trailer(&self, string_like: &mut impl StringLike, random_tag: &str) -> bool;
}

#[derive(Clone, Debug, Default)]
pub struct NoopRandomTrailer;

impl RandomTrailer for NoopRandomTrailer {
    fn prepare_string_with_tag(&self, _input: &mut String, _tag: &str) {}

    fn prepare_vec_with_tag(&self, _input: &mut Vec<u8>, _tag: &str) {}

    fn remove_trailer(&self, _string_like: &mut impl StringLike, _random_tag: &str) -> bool {
        false
    }
}

/// A prepared input for deserialization with a random trailer.
pub struct InputPlusTrailer<SliceType>(pub SliceType);

/// Bytes and string types, which for serde_json may suffer from trailing data
/// that wasn't present in the input.
pub trait StringLike: std::fmt::Debug {
    /// Length in bytes
    fn len(&self) -> usize;
    fn ends_with_string(&self, string: &str) -> bool;
    /// Check whether this ends in `s1 + s2`.
    fn ends_with_2_strings(&self, s1: &str, s2: &str) -> bool;
    fn truncate_to_bytes(&mut self, target_len: usize);
}

impl StringLike for &str {
    fn len(&self) -> usize {
        (*self).len()
    }

    fn ends_with_string(&self, string: &str) -> bool {
        self.ends_with(string)
    }

    fn ends_with_2_strings(&self, s1: &str, s2: &str) -> bool {
        self.ends_with(s2) && self[0..self.len() - s2.len()].ends_with(s1)
    }

    fn truncate_to_bytes(&mut self, target_len: usize) {
        *self = &self[..target_len];
    }
}

impl StringLike for &[u8] {
    fn len(&self) -> usize {
        (*self).len()
    }

    fn ends_with_string(&self, string: &str) -> bool {
        self.ends_with(string.as_bytes())
    }

    fn ends_with_2_strings(&self, s1: &str, s2: &str) -> bool {
        self.ends_with(s2.as_bytes()) && self[0..self.len() - s2.len()].ends_with(s1.as_bytes())
    }

    fn truncate_to_bytes(&mut self, target_len: usize) {
        *self = &self[..target_len];
    }
}

impl StringLike for String {
    fn len(&self) -> usize {
        self.len()
    }

    fn ends_with_string(&self, string: &str) -> bool {
        self.ends_with(string)
    }

    fn ends_with_2_strings(&self, s1: &str, s2: &str) -> bool {
        self.ends_with(s2) && self[0..self.len() - s2.len()].ends_with(s1)
    }

    fn truncate_to_bytes(&mut self, target_len: usize) {
        self.truncate(target_len);
    }
}

impl StringLike for Vec<u8> {
    fn len(&self) -> usize {
        self.len()
    }

    fn ends_with_string(&self, string: &str) -> bool {
        self.ends_with(string.as_bytes())
    }

    fn ends_with_2_strings(&self, s1: &str, s2: &str) -> bool {
        self.ends_with(s2.as_bytes()) && self[0..self.len() - s2.len()].ends_with(s1.as_bytes())
    }

    fn truncate_to_bytes(&mut self, target_len: usize) {
        self.truncate(target_len);
    }
}

#[cfg(test)]
impl StringLike for std::borrow::Cow<'_, str> {
    fn len(&self) -> usize {
        (**self).len()
    }

    fn ends_with_string(&self, string: &str) -> bool {
        self.ends_with(string)
    }

    fn ends_with_2_strings(&self, s1: &str, s2: &str) -> bool {
        self.ends_with(s2) && self[0..self.len() - s2.len()].ends_with(s1)
    }

    fn truncate_to_bytes(&mut self, target_len: usize) {
        match self {
            std::borrow::Cow::Borrowed(slice) => {
                slice.truncate_to_bytes(target_len);
            }
            std::borrow::Cow::Owned(string) => {
                string.truncate_to_bytes(target_len);
            }
        }
    }
}
