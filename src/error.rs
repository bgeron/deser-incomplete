use std::fmt::Display;

use serde::de::{Expected, Unexpected};

#[cfg(doc)]
use serde::de::{DeserializeSeed, Deserializer};

/// Either `DeserializerErr` or [`InternalError`] or [`InconsistentDeserializerError`].
///
/// Additional variants may be added in the future.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error<DeserializerErr> {
    // We always wrap in Box, mimicing serde_json. This is supposed to
    // make the return types of lots of intermediate functions smaller
    // and therefore faster. I don't know if this also matters for
    // deser-incomplete.
    err: Box<ErrorImpl<DeserializerErr>>,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ErrorImpl<DeserializerErr> {
    /// The wrapped deserializer returned an error.
    #[error(transparent)]
    Deserializer(DeserializerErr),
    #[error(transparent)]
    Internal(InternalError),
    /// The deserializer behaved in an inconsistent / nondeterministic way.
    #[error(transparent)]
    InconsistentDeserializer(InconsistentDeserializerError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum InternalError {
    #[error(
        "the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"
    )]
    TooManyBacktracks,
    #[error(
        "could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after {after_backtracks} backtracks)"
    )]
    NoPotentialBacktrackPoint { after_backtracks: usize },
    #[error("bug in {pkg} (please report): {0}", pkg = std::env!("CARGO_PKG_NAME"))]
    Bug(BugError),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct BugError(BugEnum);

#[derive(Debug, thiserror::Error)]
pub(crate) enum BugEnum {
    #[error("our visitor should store the obtained value on the stack, but it's missing")]
    OkButValueMissingFromStack,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum InconsistentDeserializerError {}

#[derive(Debug, thiserror::Error)]
pub enum FallbackError {
    #[error("The fallback took the visitor to compute a value, but didn't return a Result.")]
    FallbackDidntCompute,
    #[error("While constructing a fallback value: {0}")]
    FallbackVisitor(serde::de::value::Error),
}

impl<DeserializerErr> Error<DeserializerErr> {
    pub(crate) fn from_de(err: DeserializerErr) -> Self {
        Self {
            err: Box::new(ErrorImpl::Deserializer(err)),
        }
    }

    /// Was it an error from the wrapped deserializer?
    pub fn as_deserializer_error(&self) -> Option<&DeserializerErr> {
        match &*self.err {
            ErrorImpl::Deserializer(err) => Some(err),
            _ => None,
        }
    }

    pub fn into_deserializer_error(self) -> Option<DeserializerErr> {
        match *self.err {
            ErrorImpl::Deserializer(err) => Some(err),
            _ => None,
        }
    }

    pub fn as_internal_error(&self) -> Option<&InternalError> {
        match &*self.err {
            ErrorImpl::Internal(err) => Some(err),
            _ => None,
        }
    }

    pub fn into_internal_error(self) -> Option<InternalError> {
        match *self.err {
            ErrorImpl::Internal(err) => Some(err),
            _ => None,
        }
    }

    /// Was the deserializer being inconsistent?
    pub fn as_inconsistent_deserializer_error(&self) -> Option<&InconsistentDeserializerError> {
        match &*self.err {
            ErrorImpl::InconsistentDeserializer(err) => Some(err),
            _ => None,
        }
    }

    pub fn into_inconsistent_deserializer_error(self) -> Option<InconsistentDeserializerError> {
        match *self.err {
            ErrorImpl::InconsistentDeserializer(err) => Some(err),
            _ => None,
        }
    }
}

impl<DeserializerErr> Error<DeserializerErr>
where
    DeserializerErr: serde::de::Error,
{
    /// In some situations, we need to carry an error across the external
    /// [`Deserializer`]'s error type. So, in case this error was not originally
    /// a deserializer error, then make this a custom error.
    ///
    /// This is necessary for wrapping [`DeserializeSeed`].
    pub fn unpack_or_make_custom(self) -> DeserializerErr {
        match *self.err {
            ErrorImpl::Deserializer(err) => err,
            _ => DeserializerErr::custom(format!("{}: {self}", std::env!("CARGO_PKG_NAME"))),
        }
    }
}

impl<DeserializerErr> From<InternalError> for Error<DeserializerErr> {
    fn from(err: InternalError) -> Self {
        Self {
            err: Box::new(ErrorImpl::Internal(err)),
        }
    }
}

impl From<BugEnum> for InternalError {
    fn from(err: BugEnum) -> Self {
        InternalError::Bug(BugError(err))
    }
}

impl<DeserializerErr> From<BugEnum> for Error<DeserializerErr> {
    fn from(err: BugEnum) -> Self {
        Self {
            err: Box::new(ErrorImpl::Internal(err.into())),
        }
    }
}

impl<DeserializerErr> From<InconsistentDeserializerError> for Error<DeserializerErr> {
    fn from(err: InconsistentDeserializerError) -> Self {
        Self {
            err: Box::new(ErrorImpl::InconsistentDeserializer(err)),
        }
    }
}

impl<DeserializerErr> serde::de::Error for Error<DeserializerErr>
where
    DeserializerErr: serde::de::Error,
{
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::from_de(DeserializerErr::custom(msg))
    }

    #[cold]
    fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self {
        Self::from_de(DeserializerErr::invalid_type(unexp, exp))
    }

    #[cold]
    fn invalid_value(unexp: Unexpected, exp: &dyn Expected) -> Self {
        Self::from_de(DeserializerErr::invalid_value(unexp, exp))
    }

    #[cold]
    fn invalid_length(len: usize, exp: &dyn Expected) -> Self {
        Self::from_de(DeserializerErr::invalid_length(len, exp))
    }

    #[cold]
    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Self::from_de(DeserializerErr::unknown_variant(variant, expected))
    }

    #[cold]
    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Self::from_de(DeserializerErr::unknown_field(field, expected))
    }

    #[cold]
    fn missing_field(field: &'static str) -> Self {
        Self::from_de(DeserializerErr::missing_field(field))
    }

    #[cold]
    fn duplicate_field(field: &'static str) -> Self {
        Self::from_de(DeserializerErr::duplicate_field(field))
    }
}

impl<DeserializerErr> Error<DeserializerErr>
where
    DeserializerErr: std::error::Error + Send + Sync + 'static,
{
    pub fn erase(self) -> Error<Box<dyn std::error::Error + Send + Sync>> {
        let inner = *self.err;

        let err = Box::new(match inner {
            ErrorImpl::Deserializer(err) => {
                ErrorImpl::Deserializer(Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
            }
            ErrorImpl::Internal(err) => ErrorImpl::Internal(err),
            ErrorImpl::InconsistentDeserializer(err) => ErrorImpl::InconsistentDeserializer(err),
        });

        Error { err }
    }
}
