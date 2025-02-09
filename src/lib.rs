#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(
    not(all(feature = "rand", feature = "tracing")),
    allow(unused_variables, unused_imports, dead_code, unused_mut)
)]

//! # Deserialize incomplete or broken data with Serde
//!
//! Parse incomplete or broken data with existing Serde data formats.
//!
//! This is nice for ingesting streaming JSON, which is technically invalid until
//! the stream is done. By tolerating premature end of input, we can immediately make use
//! of the streaming input.
//!
//! <img src="https://raw.githubusercontent.com/bgeron/deser-incomplete/rendered/assets/live-travel-modes.gif" alt='Someone is slowly
//! typing JSON into a terminal program. The JSON is an array of objects.
//! The program gradually renders the JSON input as Rust debug output, and as a table.
//! The fields of the Rust struct are printed even though they are missing in the JSON input.
//! The example program is called "live".' title="Demo that shows parsing JSON as it is typed by the user"
//! width="60%" height="60%">
//!
//! Here, we wrapped [`serde_json`] with `deser-incomplete`, and printed the Rust
//! debug representation of the result. We also reserialized to JSON and
//! let nushell do its beautiful table formatting.
//!
//! The JSON can also come from an external program. Here is a demo program that
//! computes disk usage of directories and outputs the results as JSON.
//! In true Unix style,  displaying for the user is a separate concern,
//! implemented by a separate program.
//!
//! <img src="https://raw.githubusercontent.com/bgeron/deser-incomplete/rendered/assets/du-live.gif" alt='A Unix pipeline with
//! two programs is shown. The source program computes the disk size
//! of a bunch of directories and outputs a JSON array of objects. The sink program
//! pretty-prints the JSON table. Computing the disk size takes a while, and you can
//! see which directory is being analyzed because the result for that directory is empty
//! while it is computing.' title='Demo that shows parsing JSON as it is generated live from another program that mimics du'
//! width="60%" height="60%">
//!
//! `deser-incomplete` sits between `#[serde(Deserialize)]` and the data format. When a parse
//! error is detected (presumably because the input ended), it safely halts parsing.
//!
//! <img src="https://raw.githubusercontent.com/bgeron/deser-incomplete/rendered/assets/deser-incomplete-blocks-errors.png" alt='This library sits
//! in between Deserialize and Deserializer. Information about the parsed data is successfully
//! sent from Deserializer through deser-incomplete to Deserialize. But errors from Deserializer are
//! blocked.' width="60%" height="60%">
//!
//! ## How to use: JSON and YAML
//!
//! ```
//! let result: Result<Vec<u32>, deser_incomplete::Error<serde_json::Error>>
//!     = deser_incomplete::from_json_str("[3, 4, ");
//!
//! assert_eq!(result.unwrap(), vec![3, 4]);
//!
//! let result: Result<Vec<bool>, deser_incomplete::Error<serde_yaml::Error>>
//!    = deser_incomplete::from_yaml_str("- true\n- false\n- ");
//!
//! assert_eq!(result.unwrap(), vec![true, false]);
//! ```
//!
//! Command line:
//!
//! ```sh
//! $ cargo install deser-incomplete --example repair-deser
//!
//! $ echo '[3, 4' | repair-deser    # JSON by default
//! [3,4]
//! ```
//!
//! ## How to use: other data formats
//!
//! - You need to explain how to create the [`Deserializer`] by implementing [`Source`].
//!
//!   - If your format has `&mut T: Deserializer` then mimic [`source::JsonStr`].
//!   - If your format has `T: Deserializer` then mimic [`source::YamlStr`].
//!
//! - Some formats need a trailer for best results. For example, [`from_json_str`] appends
//!   a double-quote to the input before parsing, this lets [`serde_json`] see strings that weren't
//!   actually complete.
//!
//!   We also preprocess the input in [`from_yaml_str`], actually there it is even more important
//!   for good results.
//!
//!   _Add preprocessing with [`Options::set_random_trailer`], or turn it off such preprocessing
//!   with [`Options::disable_random_tag`]. You can see the effect of it with
//!   `cargo run --example live -- --use-random-trailer false`._
//!
//!   I expect that binary formats don't need this preprocessing.
//!
//!
//! ## How this works internally
//!
//! The implementation sits in between [`Deserialize`], [`Deserializer`], and [`Visitor`],
//! gathers metadata during the parse, and saves successful sub-parses. It also "backtracks":
//! if a parse fails, then we retry, but just before the failure point we swap out the real
//! [`Deserializer`] for a decoy which can brings deserialization to a safe end.
//!
//!
//! We apply multiple techniques. Suppose we want to parse `Vec<u32>` with [`serde_json`].
//! Here are the main techniques.
//!
//! 1. **(Example: parse empty JSON as `[]` .)** — On the top level, if parsing fails immediately (e.g.
//!    empty input) but a sequence is expected, then return `[]`.
//!
//!    _\[setting name: fallback_seq_empty_at_root]_
//!
//! 2. **(Example: parse JSON `"[3"` as `[3]` .)** — When there are no more elements in a sequence,
//!    let the [`Visitor`] construct the `Vec<u32>` and put it somewhere safe. Now
//!    `serde_json::Deserializer::deserialize_seq` notices the missing close bracket and
//!    returns `Err` to us. We ignore `Err`, retrieve the saved value again, and return `Ok`
//!    of it.
//!
//!    This happens for every `deserialize_*` method, not just sequences.
//!
//!    _\[setting name: tolerate_deserializer_fail_after_visit_success]_
//!
//! 3. **(Example: parse JSON `"[3,"` as `[3]` .)** — Inside a sequence, if parsing the next element will
//!    fail, then don't even try.
//!
//!    This works using backtracking.
//!
//!    _\[setting name: backtrack_seq_skip_item]_
//!
//! 4. Before deserializing, we append a random trailer.
//!
//! #### Random trailer
//!
//! Additionally we have a "random trailer" technique to get incomplete strings to parse.
//! Unfortunately this technique is specific to the data format. This library implements
//! it for JSON and YAML.
//!
//! This technique is not applied by default for other data formats. Even with JSON/YAML, this
//! technique can be turned off with [`Options::disable_random_tag`].
//!
//! #### Random trailer for JSON
//!
//! We actually [append][append-impl] `tRANDOM"` to every JSON input, where `RANDOM` are some randomly chosen
//! letters. It turns out that [`serde_json`] can parse any prefix of valid JSON, as long
//! as we concatenate `tRANDOM"` to it. Some examples:
//!
//! 1. **(Example: `"hello` .)** The concatenation is `"hellotRANDOM"` and we actually get
//!     this back from [`serde_json`] through `fn visit_borrowed_str` --- after [`serde_json`]
//!     removed the double-quotes.
//!
//!     In `fn visit_borrowed_str`, we notice that the string ends in `RANDOM`. Because this
//!     is a random string of letters, it cannot have been part of the incomplete JSON input.
//!     We remove the `tRANDOM` suffix and get back just `"hello"`.
//!
//! 2. **(Example: `"hello\` --- perhaps breaking in the middle of `\n` .)** The concatenation
//!     is `"hello\tRANDOM"`; the `\t` parses to a tab character. We strip off `<TAB>random`
//!     and again return `"hello"`.
//!
//! 3. **(Example: `"hello"` .)** The concatenation is `"hello"tRANDOM"`. Now [`serde_json`]
//!     visits the `hello` string as it would normally do, and if there should be any error
//!     after the visit, we can recover from it anyway as
//!     per _tolerate_deserializer_fail_after_visit_success_.
//!
//! [append-impl]: https://github.com/bgeron/deser-incomplete/blob/main/src/random_trailer/json.rs
//!
//! #### Inspecting at runtime
//!
//! There is extensive logging through the [`tracing`] library, which becomes visible if you
//! initialize the library.
//!
//! #### Guiding principles
//!
//! The logic was hand-tweaked to the following criteria:
//!
//! 1. ("soundness") For any complete and valid JSON/YAML, if you call `deser-incomplete`
//!    on a prefix, then its output should not contain data that doesn't exist in the
//!    complete JSON/YAML.
//!
//! 2. ("monotone") A larger prefix should not parse to a shorter output.
//!
//! 3. ("prompt") Ideally, each prefix contains as much data as we can be certain of.
//!
//! The implementation of [`Deserializer`] (data format) may influence the quality of the output,
//! but the default ruleset does generally very well with [`serde_json`] and [`serde_yaml`].
//!
//! There are [extensive snapshot tests][snapshot-tests] that validate the quality of the output
//! on these criteria.
//!
//! If you are curious, then it is possible to tweak the ruleset
//! with `unstable::UnstableCustomBehavior`. We also have snapshot tests for some alternative
//! parsing configurations.
//!
//! [snapshot-tests]: https://github.com/bgeron/deser-incomplete/blob/main/tests/output/json_output/seq.rs
//!
//! ## Notes and limitations
//!
//! - Ideally, your data format should be relatively greedy, in the sense that it
//!   generates information quickly and does not need to look ahead in the serialized
//!   stream too much.
//!
//! - This approach lets us safely abort parsing and get a value, but
//!   we cannot skip over invalid segments of input. (For that you need
//!   an approach like [tree-sitter](https://tree-sitter.github.io/).)
//!
//! - We cannot distinguish eof from invalid input.
//!
//! - YAML works well in general, but it is a bit less exhaustively tested than JSON.
//!   The randomized trailer is really important for YAML.
//!
//! - JSON: when parsing a floating-point number, if the end of input happens to fall
//!   directly after the decimal point, then the number is missing from the output.
//!
//! - For YAML, the randomized trailer uses a heuristic to see if we are currently in
//!   an escape sequence in a string --- but this heuristic can fail. In this case,
//!   the incomplete string will be missing from the output.
//!
//! Have fun!
//!
//! ## Acknowledgements
//!
//! Thanks to Annisa Chand and @XAMPPRocky for useful feedback.

macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        ::tracing::error!($($arg)*)
    };
}
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        ::tracing::debug!($($arg)*)
    };
}
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "tracing")]
        ::tracing::trace!($($arg)*)
    };
}

mod attempt;
mod deserialize;
pub mod error;
mod fallback;
mod options_impl;
#[cfg(feature = "rand")]
pub mod random_trailer;
#[cfg(not(feature = "rand"))]
mod random_trailer;
mod reporter;
pub mod source;
mod state;
mod util;

/// Types and traits that have to be public to satisfy rustc/rustdoc.
///
/// Instead of looking here, look at the methods of [`crate::Options`].
pub mod options {
    #[cfg(all(feature = "rand", feature = "serde_json"))]
    pub use crate::options_impl::JsonExtraOptions;
    #[cfg(all(feature = "rand", feature = "serde_yaml"))]
    pub use crate::options_impl::YamlExtraOptions;
    pub use crate::options_impl::{
        DefaultExtraOptions, ExtraOptions, MakeDefaultFallbacks, MakeDefaultReporter,
    };
}

/// Import from this crate in this library. That way, doc links work properly.
#[cfg(not(feature = "unstable"))]
mod unstable {
    pub use crate::collection_of_unstable_stuff::*;
}
/// Stuff that is not polished or likely to change.
#[cfg(feature = "unstable")]
pub mod unstable {
    pub use crate::collection_of_unstable_stuff::*;
}

#[allow(unused_imports)]
mod collection_of_unstable_stuff {
    pub use crate::fallback::Fallbacks;
    pub use crate::options_impl::{
        ExtraOptions, ExtraOptionsStruct, MakeFallbackProvider, MakeReporter,
        UnstableCustomBehavior,
    };
    pub use crate::reporter::{DefaultReporter, Reporter};
    pub(crate) trait ExtraOptionsIsUnstable {}
}

use std::borrow::Cow;

pub use error::Error;
pub use options_impl::Options;
use options_impl::UnstableCustomBehavior;
#[cfg(doc)]
use serde::{de::Visitor, Deserialize, Deserializer};
pub use source::Source;

/// Main function. Robustly deserialize incomplete input with [`serde_json`].
///
/// See methods on [`Options`] for more generic APIs.
#[cfg(all(feature = "rand", feature = "serde_json"))]
pub fn from_json_str<T>(json: &str) -> Result<T, Error<serde_json::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    Options::new_json().deserialize_from_json_str(Cow::Borrowed(json))
}

/// Like [`from_json_str`], but for bytes.
///
/// See methods on [`Options`] for more generic APIs.
#[cfg(all(feature = "rand", feature = "serde_json"))]
pub fn from_json_slice<T>(json: &[u8]) -> Result<T, Error<serde_json::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    Options::new_json().deserialize_from_json_slice(Cow::Borrowed(json))
}

/// Robustly deserialize incomplete input with [`serde_yaml`].
///
/// See methods on [`Options`] for more generic APIs.
#[cfg(all(feature = "rand", feature = "serde_yaml"))]
pub fn from_yaml_str<T>(yaml: &str) -> Result<T, Error<serde_yaml::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    Options::new_yaml().deserialize_from_yaml_str(Cow::Borrowed(yaml))
}

/// Like [`from_yaml_str`], but for bytes.
///
/// See methods on [`Options`] for more generic APIs.
#[cfg(all(feature = "rand", feature = "serde_yaml"))]
pub fn from_yaml_slice<T>(yaml: &[u8]) -> Result<T, Error<serde_yaml::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    Options::new_yaml().deserialize_from_yaml_slice(Cow::Borrowed(yaml))
}
