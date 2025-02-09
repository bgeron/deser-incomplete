use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

use clap::ValueEnum;
use deser_incomplete::options::ExtraOptions;
use deser_incomplete::{Error, Options};
use serde::Deserialize;
use tap::Pipe;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum Format {
    #[default]
    SerdeJson,
    SerdeYaml,
}

pub type ParseOk = Arc<dyn Parsed>;
pub type ParseResult = Result<ParseOk, Error<Box<dyn std::error::Error + Send + Sync>>>;

pub struct ParseSettings {
    pub use_random_trailer: bool,
}

impl Format {
    pub fn parse<P>(&self, settings: &ParseSettings, input: &[u8]) -> ParseResult
    where
        P: for<'de> Deserialize<'de> + Parsed + 'static,
    {
        match self {
            Format::SerdeJson => deser_incomplete::Options::new_json()
                .pipe(|options| apply_settings(settings, options))
                .deserialize_from_json_slice::<P>(Cow::Borrowed(input))
                .map(|ok| Arc::new(ok) as Arc<dyn Parsed>)
                .map_err(Error::erase),

            #[cfg(feature = "serde_yaml")]
            Format::SerdeYaml => deser_incomplete::Options::new_yaml()
                .pipe(|options| apply_settings(settings, options))
                .deserialize_from_yaml_slice::<P>(Cow::Borrowed(input))
                .map(|ok| Arc::new(ok) as Arc<dyn Parsed>)
                .map_err(Error::erase),

            #[cfg(not(feature = "serde_yaml"))]
            Format::SerdeYaml => {
                panic!(
                    "Please enable --features serde_yaml to parse YAML, or run cargo with --all-features)"
                )
            }
        }
    }
}

fn apply_settings<Extra: ExtraOptions>(
    settings: &ParseSettings,
    options: Options<Extra>,
) -> Options<Extra> {
    if settings.use_random_trailer {
        options
    } else {
        options.disable_random_tag()
    }
}

pub trait Parsed: Debug + erased_serde::Serialize + Send + Sync {}
erased_serde::serialize_trait_object!(Parsed);

impl<T> Parsed for T where T: Debug + serde::Serialize + Send + Sync {}
