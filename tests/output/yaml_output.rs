use std::borrow::Cow;
use std::fmt::Debug;

use deser_incomplete::options::YamlExtraOptions;
use deser_incomplete::unstable::UnstableCustomBehavior;
use deser_incomplete::Options;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::run_on_prefixes_and_format_outputs;

mod any;
mod bool;
mod borrowed_string;
mod r#enum;
mod error;
mod map;
mod newtype_struct;
mod number;
mod other;
mod seq;
mod string;
mod r#struct;
mod tuple_struct;
mod unit_struct;

type BoxSerialize = Box<dyn erased_serde::Serialize>;

/// Robustly deserialize all prefixes of the input as YAML.
///
/// The output is deduplicated -- only inputs are shown where the output changes.
///
/// We also check if our wrapping of [`serde_yaml`] on the whole string matches
/// [`serde_yaml`] without wrapping.
#[allow(clippy::type_complexity)]
pub(crate) fn run_yaml_modes_on_prefixes_and_format_outputs<
    'input,
    T: for<'de> Deserialize<'de> + Serialize + Debug + PartialEq + 'static,
>(
    modes: &[(&'static str, Options<YamlExtraOptions>)],
    full_input: &'input impl AsRef<[u8]>,
) -> IndexMap<&'input str, IndexMap<Cow<'input, str>, BoxSerialize>> {
    let full_input = full_input.as_ref();

    let reference_official: Option<T> = serde_yaml::from_slice(full_input).ok();

    modes
        .iter()
        .map(|(mode_desc, options)| {
            let inputs_outputs = run_on_prefixes_and_format_outputs(full_input, |inp| {
                options
                    .clone()
                    .deserialize_from_yaml_slice::<T>(Cow::Borrowed(inp))
                    .map_err(|err| err.to_string())
            });

            let last_output_matches_serde_yaml_friendly =
                match reference_official.as_ref().map(|reference| {
                    Ok(reference)
                        == inputs_outputs
                            .last()
                            .expect("every slice has a prefix")
                            .1
                            .as_ref()
                }) {
                    Some(true) => None,
                    Some(false) => Some("no"),
                    None => Some("serde_yaml failed"),
                };
            let trailing_line = (last_output_matches_serde_yaml_friendly).map(|friendly| {
                (
                    Cow::Borrowed("final output matches serde_yaml?"),
                    Box::new(friendly) as BoxSerialize,
                )
            });

            let lines: IndexMap<_, _> = inputs_outputs
                .into_iter()
                .map(|(input, output)| -> (_, Box<dyn erased_serde::Serialize>) {
                    (input, Box::new(output))
                })
                .chain(trailing_line)
                .collect();

            (*mode_desc, lines)
        })
        .collect()
}

fn default_modes() -> Vec<(&'static str, Options<YamlExtraOptions>)> {
    vec![
        ("default behavior", Options::new_yaml()),
        (
            "default behavior except no randomized trailer",
            deser_incomplete::Options::new_yaml().disable_random_tag(),
        ),
        (
            "default behavior, 0 backtracks",
            Options::new_yaml().with_max_n_backtracks(Some(0)),
        ),
        (
            "no fallbacks, 0 backtracks",
            Options::new_yaml()
                .custom_behavior(UnstableCustomBehavior::default().no_fallbacks())
                .with_max_n_backtracks(Some(0)),
        ),
        (
            "no fallbacks, 1 backtracks",
            Options::new_yaml()
                .custom_behavior(UnstableCustomBehavior::default().no_fallbacks())
                .with_max_n_backtracks(Some(1)),
        ),
        (
            "default behavior, 1 backtracks",
            Options::new_yaml().with_max_n_backtracks(Some(1)),
        ),
        (
            "strict behavior",
            Options::new_yaml().custom_behavior(UnstableCustomBehavior::strict()),
        ),
    ]
}
