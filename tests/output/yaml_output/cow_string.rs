use std::marker::PhantomData;

use serde::Serialize;

/// Like `Cow<str>`, but deserializes by saving a deserialized string when possible.
///
/// Serializes as an enum... that's different! For testing.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum CowString<'a> {
    VisitBorrowedStr(&'a str),
    VisitStr { cloned: String },
    VisitString(String),
}

struct CowStrVisitor<'a>(PhantomData<&'a ()>);

impl<'de> serde::Deserialize<'de> for CowString<'de> {
    fn deserialize<D>(deserializer: D) -> Result<CowString<'de>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(CowStrVisitor::<'de>(PhantomData))
    }
}

impl<'de> serde::de::Visitor<'de> for CowStrVisitor<'de> {
    type Value = CowString<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<CowString<'de>, E>
    where
        E: serde::de::Error,
    {
        Ok(CowString::VisitBorrowedStr(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<CowString<'de>, E>
    where
        E: serde::de::Error,
    {
        Ok(CowString::VisitStr {
            cloned: value.to_owned(),
        })
    }

    fn visit_string<E>(self, value: String) -> Result<CowString<'de>, E>
    where
        E: serde::de::Error,
    {
        Ok(CowString::VisitString(value))
    }
}
