use std::borrow::Cow;
use std::marker::PhantomData;
#[cfg(feature = "rand")]
use std::sync::Arc;

use serde::de::DeserializeSeed;
#[cfg(doc)]
use serde::de::{Deserializer, EnumAccess, MapAccess, Visitor};
use serde::Deserialize;

pub use crate::error::Error;
use crate::error::InternalError;
use crate::fallback::{DefaultFallbacks, Fallbacks};
pub use crate::random_trailer::RandomTrailer;
use crate::random_trailer::{InputPlusTrailer, NoopRandomTrailer, StringLike};
use crate::state::AttemptState;
use crate::unstable::{DefaultReporter, ExtraOptionsIsUnstable};
use crate::Source;

/// Number of times that we may backtrack.
///
/// For good results, you should allow at least one backtracking for when the input
/// stops in the middle of a map/struct value or enum.
///
/// A higher limit on backtracks is useful when not all struct fields
/// are declared `#[serde(default)]`. In this case, the algorithm will attempt to
/// incrementally prune on higher levels, e.g. omitting the list item that contains
/// the end-of-file, or omitting a field of an enclosing struct.
const DEFAULT_MAX_BACKTRACKS: Option<usize> = Some(10);

#[cfg(feature = "rand")]
const RANDOM_TAG_LEN: usize = 8;

/// Options for deserialization.
///
/// The most important methods are:
///
/// - [`Options::deserialize_from_json_str`] for JSON,
/// - [`Options::deserialize_from_yaml_str`] for YAML,
/// - [`Options::deserialize_source`] for a generic source.
#[derive(Clone, Debug)]
pub struct Options<Extra: ExtraOptions = DefaultExtraOptions> {
    /// This is a random string that forms part of a suffix we add to
    /// the input, for some data types.
    ///
    /// As of Dec 2024, we don't stabilize the specific string format.
    #[cfg(feature = "rand")]
    random_tag: Option<Arc<str>>,

    pub(crate) max_n_backtracks: Option<usize>,

    pub(crate) behavior: UnstableCustomBehavior,

    pub(crate) extra: Extra,
}

impl Options {
    /// Default config for JSON.
    ///
    /// This will currently generate a short extra trailer on inputs
    /// for improved deserialization of incomplete JSON.
    #[cfg(all(feature = "rand", feature = "serde_json"))]
    pub fn new_json() -> Options<JsonExtraOptions> {
        let base = Options {
            ..Options::new_nonce()
        };
        base.set_random_trailer(crate::random_trailer::json::JsonRandomTrailer)
    }

    /// Default config for YAML.
    ///
    /// This will currently generate a short extra trailer on inputs
    /// for improved deserialization of incomplete YAML.
    ///
    /// For YAML in particular, this suffix is important to get
    /// good behavior.
    #[cfg(all(feature = "rand", feature = "serde_yaml"))]
    pub fn new_yaml() -> Options<YamlExtraOptions> {
        let base = Options {
            ..Options::new_nonce()
        };
        base.set_random_trailer(crate::random_trailer::yaml::YamlRandomTrailer)
    }

    /// Basic config, suitable for any data format.
    ///
    /// These options support adding a randomized trailer to the input.
    /// However, you should probably call [`Options::set_random_trailer`]
    /// to specify how this trailer should be removed from parsed strings.
    #[cfg(feature = "rand")]
    pub fn new_nonce() -> Options<DefaultExtraOptions> {
        use rand::distributions::{Alphanumeric, DistString};
        use rand::thread_rng;

        // In the future, this may change to only generate a single random
        // tag for the lifetime of the application.
        let tag = Alphanumeric.sample_string(&mut thread_rng(), RANDOM_TAG_LEN);
        Options {
            random_tag: Some(tag.into()),
            ..Options::new_no_nonce()
        }
    }

    /// Basic config, suitable for any data format. However, this
    /// config does not allow adding a randomized trailer to the input,
    /// which tends to benefit many formats.
    ///
    /// - For `serde_json`, this means you won't get incomplete strings deserialized
    ///
    /// - For `serde_yaml`, this means that your output will flicker, as it seems to
    ///   buffer lines somehow, and if a line has an unterminated string, then the
    ///   whole line will be missing.
    pub fn new_no_nonce() -> Options<DefaultExtraOptions> {
        Options {
            #[cfg(feature = "rand")]
            random_tag: None,
            max_n_backtracks: DEFAULT_MAX_BACKTRACKS,
            behavior: UnstableCustomBehavior::default(),
            extra: DefaultExtraOptions::default(),
        }
    }
}

impl<Extra: ExtraOptions> Options<Extra> {
    pub fn with_max_n_backtracks(mut self, max_n_backtracks: Option<usize>) -> Self {
        self.max_n_backtracks = max_n_backtracks;
        self
    }

    /// Like [`crate::from_json_str`], but with options. This applies the random trailer.
    #[cfg(all(feature = "rand", feature = "serde_json"))]
    pub fn deserialize_from_json_str<T>(self, json: Cow<str>) -> Result<T, Error<serde_json::Error>>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let prepared = self.prepare_str_for_borrowed_deserialization(json);
        self.deserialize_from_json_str_borrowed(&prepared)
    }

    /// Like [`crate::from_json_slice`], but with options. This applies the random trailer.
    #[cfg(all(feature = "rand", feature = "serde_json"))]
    pub fn deserialize_from_json_slice<T>(
        self,
        json: Cow<[u8]>,
    ) -> Result<T, Error<serde_json::Error>>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let prepared = self.prepare_slice_for_borrowed_deserialization(json);
        self.deserialize_from_json_slice_borrowed(&prepared)
    }

    /// Like [`crate::from_yaml_str`], but with options. This applies the random trailer.
    #[cfg(all(feature = "rand", feature = "serde_yaml"))]
    pub fn deserialize_from_yaml_str<T>(self, yaml: Cow<str>) -> Result<T, Error<serde_yaml::Error>>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let prepared = self.prepare_str_for_borrowed_deserialization(yaml);
        self.deserialize_from_yaml_str_borrowed(&prepared)
    }

    /// Like [`crate::from_yaml_slice`], but with options. This applies the random trailer.
    #[cfg(all(feature = "rand", feature = "serde_yaml"))]
    pub fn deserialize_from_yaml_slice<T>(
        self,
        yaml: Cow<[u8]>,
    ) -> Result<T, Error<serde_yaml::Error>>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let prepared = self.prepare_slice_for_borrowed_deserialization(yaml);
        self.deserialize_from_yaml_slice_borrowed(&prepared)
    }

    /// Like [`Self::deserialize_from_json_slice`], but can deserialize borrowed strings and return them
    /// directly.
    ///
    /// This comes at the cost that we cannot use the random trailer technique that gives
    /// us access to the contents of incomplete strings.
    ///
    /// If you need incomplete strings as well, then use [`Self::deserialize_from_json_slice_borrowed`].
    ///
    /// ```
    /// # use serde::Deserialize;
    /// #[derive(Debug, Deserialize, PartialEq)]
    /// struct TravelMode {
    ///    #[serde(default)]
    ///    mode: String,
    ///    benefit: Option<String>
    /// }
    ///
    /// let json = r#"[{"mode": "foot", "benefit": "healthy"}, {"mode": "incomplete"#;
    /// let modes: Vec<TravelMode> = deser_incomplete::Options::new_json().deserialize_from_json_slice_plain_return_borrowed(&json).unwrap();
    /// assert_eq!(modes, [
    ///    TravelMode { mode: "foot".to_string(), benefit: Some("healthy".to_string()) },
    ///    TravelMode { mode: "".to_string(), benefit: None },
    ///    // Note: this function fails on incomplete strings, because
    ///    // the randomized trailer is needed for those.
    /// ]);
    /// ```
    #[cfg(feature = "serde_json")]
    pub fn deserialize_from_json_slice_plain_return_borrowed<'de, T>(
        self,
        json: &'de impl AsRef<[u8]>,
    ) -> Result<T, Error<serde_json::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(crate::source::JsonBytes(json.as_ref()))
    }

    /// Advanced API. Lets you deserialize into borrowed types like `&str`, while supporting
    /// the random trailer that gives us access to the contents of incomplete strings.
    ///
    /// (The difference is that this only needs `T: serde::de::Deserialize<'de>`, which is weaker.)
    ///
    /// **Note: This API is relatively likely to change (more unstable) compared to [`Self::deserialize_from_json_str`].**
    ///
    /// ```
    /// # use serde::Deserialize;
    /// /// Note: `&'a str` instead of `String`.
    /// ///
    /// /// Like with serde_json, deserializing to &str can fail. Instead, you should probably
    /// /// use `Cow<str>`, or just `String`.
    /// #[derive(Debug, Deserialize, PartialEq)]
    /// struct TravelMode<'a> {
    ///    mode: &'a str,
    ///    benefit: Option<&'a str>
    /// }
    ///
    /// let json = r#"[{"mode": "foot", "benefit": "healthy"}, {"mode": "aeropl"#;
    /// let options = deser_incomplete::Options::new_json();
    /// let prepared = options.prepare_str_for_borrowed_deserialization(json.into());
    /// let modes: Vec<TravelMode> = options.deserialize_from_json_str_borrowed(&prepared).unwrap();
    /// assert_eq!(modes, [
    ///    TravelMode { mode: "foot", benefit: Some("healthy") },
    ///    TravelMode { mode: "aeropl", benefit: None }
    /// ]);
    /// ```
    #[cfg(feature = "serde_json")]
    pub fn deserialize_from_json_str_borrowed<'de, T>(
        self,
        InputPlusTrailer(prepared_json): &'de InputPlusTrailer<impl AsRef<str>>,
    ) -> Result<T, Error<serde_json::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(crate::source::JsonStr(prepared_json.as_ref()))
    }

    /// Advanced API. See [`Self::deserialize_from_json_str_borrowed`], or
    /// use [`Self::deserialize_from_json_slice`] for a simpler API.
    ///
    /// **Note: This API is relatively likely to change (more unstable) compared to [`Self::deserialize_from_json_slice`].**
    #[cfg(feature = "serde_json")]
    pub fn deserialize_from_json_slice_borrowed<'de, T>(
        self,
        InputPlusTrailer(prepared_json): &'de InputPlusTrailer<impl AsRef<[u8]>>,
    ) -> Result<T, Error<serde_json::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(crate::source::JsonBytes(prepared_json.as_ref()))
    }

    /// Advanced API. See [`Self::deserialize_from_json_str_borrowed`], or
    /// use [`Self::deserialize_from_yaml_str`] for a simpler API.
    #[cfg(feature = "serde_yaml")]
    pub fn deserialize_from_yaml_str_borrowed<'de, T>(
        self,
        InputPlusTrailer(prepared_yaml): &'de InputPlusTrailer<impl AsRef<str>>,
    ) -> Result<T, Error<serde_yaml::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(crate::source::YamlStr(prepared_yaml.as_ref()))
    }

    /// Advanced API. See [`Self::deserialize_from_json_str_borrowed`], or
    /// use [`Self::deserialize_from_yaml_slice`] for a simpler API.
    #[cfg(feature = "serde_yaml")]
    pub fn deserialize_from_yaml_slice_borrowed<'de, T>(
        self,
        InputPlusTrailer(prepared_yaml): &'de InputPlusTrailer<impl AsRef<[u8]>>,
    ) -> Result<T, Error<serde_yaml::Error>>
    where
        T: serde::de::Deserialize<'de>,
    {
        self.deserialize_source(crate::source::YamlBytes(prepared_yaml.as_ref()))
    }

    /// Prepare a string for borrowed deserialization with a method
    /// like [`Self::deserialize_from_json_str_borrowed`], by appending the random trailer.
    ///
    /// This returns a newtype wrapper, so you can undo the effects yourself.
    #[cfg(feature = "rand")]
    pub fn prepare_str_for_borrowed_deserialization<'a>(
        &self,
        mut input: Cow<'a, str>,
    ) -> InputPlusTrailer<Cow<'a, str>> {
        use RandomTrailer as _;

        #[cfg(feature = "rand")]
        if let Some(tag) = self.random_tag.as_ref() {
            self.extra
                .get_random_trailer()
                .prepare_string_with_tag(Cow::to_mut(&mut input), tag);
        }
        InputPlusTrailer(input)
    }

    /// Prepare a slice for borrowed deserialization with a method
    /// like [`Self::deserialize_from_json_slice_borrowed`], by appending the random trailer.
    ///
    /// This returns a newtype wrapper, so you can undo the effects yourself.
    #[cfg(feature = "rand")]
    pub fn prepare_slice_for_borrowed_deserialization<'a>(
        &self,
        mut input: Cow<'a, [u8]>,
    ) -> InputPlusTrailer<Cow<'a, [u8]>> {
        use RandomTrailer as _;

        #[cfg(feature = "rand")]
        if let Some(tag) = self.random_tag.as_ref() {
            self.extra
                .get_random_trailer()
                .prepare_vec_with_tag(Cow::to_mut(&mut input), tag);
        }
        InputPlusTrailer(input)
    }

    /// Customize internal behavior.
    ///
    /// This is meant for data formats where the defaults may not work well, but
    /// it's unclear if such customization helps anywhere.
    ///
    /// ## Example
    ///
    /// ```
    /// # use deser_incomplete::Options;
    ///
    /// let mut behavior : deser_incomplete::unstable::UnstableCustomBehavior
    ///     = Default::default();
    /// behavior.fallback_any_as_none = true;
    ///
    /// let result =
    ///     Options::new_json()
    ///     .custom_behavior(behavior)
    ///     .deserialize_from_json_str::<serde_json::Value>("".into());
    ///
    /// assert_eq!(result.unwrap(), serde_json::Value::Null);
    ///
    /// // Normally, this would be Err.
    /// assert_eq!(
    ///     deser_incomplete::from_json_str::<serde_json::Value>("").unwrap_err().to_string(),
    ///     "could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)");
    /// ```
    #[cfg(feature = "unstable")]
    pub fn custom_behavior(self, behavior: UnstableCustomBehavior) -> Self {
        Options { behavior, ..self }
    }

    /// Don't use a random tag. This can make deserialization a tiny bit cheaper,
    /// because the input does not have to be reallocated.
    #[cfg(feature = "rand")]
    pub fn disable_random_tag(mut self) -> Self {
        self.random_tag = None;
        self
    }
}

#[cfg(feature = "rand")]
impl<R, F, RT> Options<ExtraOptionsStruct<R, F, RT>>
where
    R: MakeReporter,
    F: MakeFallbackProvider,
    RT: RandomTrailer,
{
    /// Set a different method for randomized trailers.
    pub fn set_random_trailer<RT2>(
        self,
        random_trailer: RT2,
    ) -> Options<ExtraOptionsStruct<R, F, RT2>>
    where
        RT2: RandomTrailer,
    {
        let Options {
            random_tag,
            max_n_backtracks,
            behavior,
            extra,
        } = self;

        Options {
            random_tag,
            max_n_backtracks,
            behavior,
            extra: ExtraOptionsStruct {
                make_reporter: extra.make_reporter,
                make_fallback_provider: extra.make_fallback_provider,
                random_trailer,
            },
        }
    }

    /// Set a way to report progress internally. (The default reporter logs on [`tracing`].)
    ///
    /// ## Example
    ///
    /// ```
    /// # use deser_incomplete::Options;
    ///
    /// let x =
    ///     Options::new_json()
    ///     .set_reporter(deser_incomplete::unstable::DefaultReporter::new())
    ///     .deserialize_from_json_str::<i32>("3".into())
    ///     .unwrap();
    ///
    /// assert_eq!(x, 3);
    /// ```
    #[cfg(feature = "unstable")]
    pub fn set_reporter<R2>(
        self,
        reporter: R2,
    ) -> Options<ExtraOptionsStruct<CustomReporter<R2>, F, RT>>
    where
        R2: crate::reporter::Reporter + Clone,
    {
        let Options {
            random_tag,
            max_n_backtracks,
            behavior,
            extra,
        } = self;

        Options {
            random_tag,
            max_n_backtracks,
            behavior,
            extra: ExtraOptionsStruct {
                make_reporter: CustomReporter(reporter),
                make_fallback_provider: extra.make_fallback_provider,
                random_trailer: extra.random_trailer,
            },
        }
    }

    /// Set a way to provide fallback values. (The default fallback provider
    /// has tuned defaults that should be reasonable for many data formats,
    /// and they are good for JSON and YAML.)
    #[doc(hidden)]
    #[cfg(feature = "unstable")]
    pub fn set_fallback_provider<F2>(
        self,
        fallback_provider: F2,
    ) -> Options<ExtraOptionsStruct<R, CustomFallbackProvider<F2>, RT>>
    where
        F2: Fallbacks + Clone,
    {
        let Options {
            random_tag,
            max_n_backtracks,
            behavior,
            extra,
        } = self;

        Options {
            random_tag,
            max_n_backtracks,
            behavior,
            extra: ExtraOptionsStruct {
                make_reporter: extra.make_reporter,
                make_fallback_provider: CustomFallbackProvider(fallback_provider),
                random_trailer: extra.random_trailer,
            },
        }
    }
}

impl<Extra: ExtraOptions> Options<Extra> {
    /// Do our best to take off any potential junk that was only added by us,
    /// caused by the random trailer.
    ///
    /// Return true if the input was modified and this value seems to be incomplete.
    #[must_use]
    pub(crate) fn remove_tag_from_stringlike(&self, stringy: &mut impl StringLike) -> bool {
        #![cfg_attr(not(feature = "rand"), allow(unused_variables))]

        #[cfg(feature = "rand")]
        {
            if let Some(tag) = self.random_tag.as_ref() {
                return self.extra.get_random_trailer().remove_trailer(stringy, tag);
            }
        }

        false
    }
}

/// Monomorphized options.
///
/// This is a parameter pack for type parameters.
///
/// All of this is unstable.
#[doc(hidden)]
#[allow(private_bounds)]
pub trait ExtraOptions: ExtraOptionsIsUnstable {
    /// Will only be called once per invocation of a public function in this crate
    fn make_reporter(&mut self) -> Self::Reporter;
    type Reporter: crate::reporter::Reporter;

    /// Will only be called once per invocation of a public function in this crate
    fn make_fallback_provider(
        &mut self,
        behavior: &UnstableCustomBehavior,
    ) -> Self::FallbackProvider;
    type FallbackProvider: Fallbacks;

    fn get_random_trailer(&self) -> &Self::RandomTrailer;
    type RandomTrailer: RandomTrailer;
}

pub type DefaultExtraOptions =
    ExtraOptionsStruct<MakeDefaultReporter, MakeDefaultFallbacks, NoopRandomTrailer>;
#[cfg(all(feature = "rand", feature = "serde_json"))]
pub type JsonExtraOptions = ExtraOptionsStruct<
    MakeDefaultReporter,
    MakeDefaultFallbacks,
    crate::random_trailer::json::JsonRandomTrailer,
>;
#[cfg(all(feature = "rand", feature = "serde_yaml"))]
pub type YamlExtraOptions = ExtraOptionsStruct<
    MakeDefaultReporter,
    MakeDefaultFallbacks,
    crate::random_trailer::yaml::YamlRandomTrailer,
>;

#[doc(hidden)]
#[derive(Debug, Clone, Default)]
pub struct ExtraOptionsStruct<MakeReporter, MakeFallbackProvider, RandomTrailer> {
    pub(crate) make_reporter: MakeReporter,
    pub(crate) make_fallback_provider: MakeFallbackProvider,
    pub(crate) random_trailer: RandomTrailer,
}

#[doc(hidden)]
pub trait MakeReporter {
    type Reporter: crate::reporter::Reporter;
    fn make_reporter(&mut self) -> Self::Reporter;
}
#[doc(hidden)]
pub trait MakeFallbackProvider {
    type FallbackProvider: Fallbacks;
    fn make_fallback_provider(
        &mut self,
        behavior: &UnstableCustomBehavior,
    ) -> Self::FallbackProvider;
}

#[doc(hidden)]
#[derive(Debug, Clone, Default)]
pub struct MakeDefaultReporter;
impl MakeReporter for MakeDefaultReporter {
    type Reporter = DefaultReporter;
    fn make_reporter(&mut self) -> Self::Reporter {
        DefaultReporter::new()
    }
}
#[doc(hidden)]
#[derive(Debug, Clone, Default)]
pub struct MakeDefaultFallbacks;
impl MakeFallbackProvider for MakeDefaultFallbacks {
    type FallbackProvider = DefaultFallbacks;
    fn make_fallback_provider(
        &mut self,
        behavior: &UnstableCustomBehavior,
    ) -> Self::FallbackProvider {
        DefaultFallbacks {
            behavior: behavior.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CustomReporter<T>(T);
impl<T: crate::reporter::Reporter + Clone> MakeReporter for CustomReporter<T> {
    type Reporter = T;
    fn make_reporter(&mut self) -> Self::Reporter {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct CustomFallbackProvider<T>(T);
impl<T: Fallbacks + Clone> MakeFallbackProvider for CustomFallbackProvider<T> {
    type FallbackProvider = T;
    fn make_fallback_provider(
        &mut self,
        _behavior: &UnstableCustomBehavior,
    ) -> Self::FallbackProvider {
        self.0.clone()
    }
}

impl<R, F, RT> ExtraOptions for ExtraOptionsStruct<R, F, RT>
where
    R: MakeReporter,
    F: MakeFallbackProvider,
    RT: RandomTrailer,
{
    fn make_reporter(&mut self) -> Self::Reporter {
        self.make_reporter.make_reporter()
    }
    type Reporter = R::Reporter;

    fn make_fallback_provider(
        &mut self,
        behavior: &UnstableCustomBehavior,
    ) -> Self::FallbackProvider {
        self.make_fallback_provider.make_fallback_provider(behavior)
    }
    type FallbackProvider = F::FallbackProvider;

    fn get_random_trailer(&self) -> &RT {
        &self.random_trailer
    }
    type RandomTrailer = RT;
}

impl<R, F, RT> ExtraOptionsIsUnstable for ExtraOptionsStruct<R, F, RT> {}

/// Customize behavior.
///
/// This both controls
///
///   - when deserializer encounters end-of-input, but we still have
///     a chance to fill in the value and succeed deserialization, then
///     we can make an educated guess based on what the data type expected
///     (which method of [`crate::Deserializer`] was called).
///
///     For instance, when deserializing an option, JSON `fa` will choose the
///     `Some` case, the deserializer will error, but can save deserialization
///     and fill in `none`.
///
///   - which points are eligible for backtracking, e.g. is it okay to retry
///     but omit a list item, a map item, or convert `Some` into `None`.
///
/// The default is an educated guess that should make many data types deserialize
/// successfully on the first backtrack, without trying to apply too many
/// fallbacks: incomplete list items are skipped, but no fallback numbers are
/// filled in.
///
/// ## Limitations
///
/// Consider that fallback values are not a good substitute for `#[serde(default)]`:
///
/// ```
/// struct Point {
///     // note: missing #[serde(default)]
///     x: i32,
///     // note: missing #[serde(default)]
///     y: i32,
/// }
/// ```
///
/// We could put a fallback of `0` for integers, and that will make `{"x": 1, "y":` parse successfully,
/// but `{"x": 1` will still not parse because all `Point`s require a value for `y`.
///
/// ## Stability
///
/// This interface is not subject to semver (as it is unstable), and may change
/// or be removed at any time.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UnstableCustomBehavior {
    /// If the visitor returns Ok but the deserializer returns an error, then return
    /// the visitor's value.
    pub tolerate_deserializer_fail_after_visit_success: bool,
    pub fallback_any_as_none: bool,
    pub fallback_ignored_any_as_none: bool,
    pub fallback_default_bool: Option<bool>,
    pub fallback_int_zero: bool,
    pub fallback_default_float: Option<f32>,
    pub fallback_default_char: Option<char>,
    pub fallback_default_str: Option<&'static str>,
    pub fallback_bytes_empty: bool,
    /// In case [`Deserializer::deserialize_option`] does not do anything, then
    /// just go in and visit a None.
    pub fallback_none: bool,
    pub fallback_none_at_mandatory: bool,
    /// In case [`Deserializer::deserialize_unit`] does not do anything, then
    /// just go in and visit a unit.
    ///
    /// ## Caveat
    ///
    /// Especially with a random trailer, this can sometimes generate spurious
    /// list elements.
    pub fallback_unit: bool,
    pub fallback_unit_at_mandatory: bool,

    /// In case [`Deserializer::deserialize_unit_struct`] does not do anything, then
    /// just go in and visit a unit struct.
    ///
    /// ## Caveat
    ///
    /// Especially with a random trailer, this can sometimes generate spurious
    /// list elements.
    pub fallback_unit_struct: bool,
    pub fallback_unit_struct_at_mandatory: bool,

    pub fallback_seq_empty: bool,
    pub fallback_seq_empty_at_root: bool,
    pub backtrack_seq_empty_for_value: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_seq_skip_item: bool,
    pub backtrack_seq_skip_item: bool,
    pub fallback_tuple_empty: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_tuple_skip_item: bool,
    pub backtrack_tuple_skip_item: bool,
    pub fallback_tuple_struct_empty: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_tuple_struct_skip_item: bool,
    pub backtrack_tuple_struct_skip_item: bool,
    pub fallback_map_empty: bool,
    pub fallback_map_empty_at_root: bool,
    pub backtrack_map_empty_for_value: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_map_skip_item: bool,
    pub backtrack_map_skip_item: bool,
    pub fallback_struct_empty: bool,
    pub fallback_struct_empty_at_root: bool,
    pub backtrack_struct_empty_for_value: bool,
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_struct_skip_field: bool,
    pub backtrack_struct_skip_field: bool,
    pub fallback_unit_variant: bool,

    /// Whether it's okay to fallback to skipping an element or field in circumstances
    /// other than [`Deserializer::deserialize_seq`] or [`Deserializer::deserialize_tuple`].
    ///
    /// `fallback_*_skip_item` are probably a bad idea, because they can mask a backtracking
    /// point that is better.
    pub fallback_other_skip_item: bool,
    pub backtrack_other_skip_item: bool,

    /// Whether incomplete strings should be allowed or rejected
    /// in [`MapAccess::next_key`] or [`EnumAccess::variant`].
    pub allow_incomplete_string_in_key_or_variant: bool,
}

impl Default for UnstableCustomBehavior {
    fn default() -> Self {
        Self {
            tolerate_deserializer_fail_after_visit_success: true,
            fallback_any_as_none: false,
            fallback_ignored_any_as_none: false,
            fallback_default_bool: None,
            fallback_int_zero: false,
            fallback_default_float: None,
            fallback_default_char: None,
            fallback_default_str: None,
            fallback_bytes_empty: false,
            fallback_none: true,
            fallback_none_at_mandatory: true,
            fallback_unit: false,
            fallback_unit_at_mandatory: true,
            fallback_unit_struct: false,
            fallback_unit_struct_at_mandatory: true,
            fallback_seq_empty: false,
            fallback_seq_empty_at_root: true,
            backtrack_seq_empty_for_value: true,
            fallback_seq_skip_item: false,
            backtrack_seq_skip_item: true,
            fallback_tuple_empty: false,
            fallback_tuple_skip_item: false,
            backtrack_tuple_skip_item: true,
            fallback_tuple_struct_empty: false,
            fallback_tuple_struct_skip_item: false,
            backtrack_tuple_struct_skip_item: true,
            fallback_map_empty: false,
            fallback_map_empty_at_root: true,
            backtrack_map_empty_for_value: true,
            fallback_map_skip_item: false,
            backtrack_map_skip_item: true,
            fallback_struct_empty: false,
            fallback_struct_empty_at_root: true,
            backtrack_struct_empty_for_value: true,
            fallback_struct_skip_field: true,
            backtrack_struct_skip_field: true,
            fallback_unit_variant: true,
            fallback_other_skip_item: false,
            backtrack_other_skip_item: true,
            allow_incomplete_string_in_key_or_variant: false,
        }
    }
}

impl UnstableCustomBehavior {
    pub fn no_fallbacks(mut self) -> Self {
        let Self {
            tolerate_deserializer_fail_after_visit_success: _,
            fallback_any_as_none,
            fallback_ignored_any_as_none,
            fallback_default_bool,
            fallback_int_zero,
            fallback_default_float,
            fallback_default_char,
            fallback_default_str,
            fallback_bytes_empty,
            fallback_none,
            fallback_none_at_mandatory,
            fallback_unit,
            fallback_unit_at_mandatory,
            fallback_unit_struct,
            fallback_unit_struct_at_mandatory,
            fallback_seq_empty,
            fallback_seq_empty_at_root,
            backtrack_seq_empty_for_value: _,
            fallback_seq_skip_item,
            backtrack_seq_skip_item: _,
            fallback_tuple_empty,
            fallback_tuple_skip_item,
            backtrack_tuple_skip_item: _,
            fallback_tuple_struct_empty,
            fallback_tuple_struct_skip_item: fallback_tuple_struct_skip_field,
            backtrack_tuple_struct_skip_item: _,
            fallback_map_empty,
            fallback_map_empty_at_root,
            backtrack_map_empty_for_value: _,
            fallback_map_skip_item,
            backtrack_map_skip_item: _,
            fallback_struct_empty,
            fallback_struct_empty_at_root,
            backtrack_struct_empty_for_value: _,
            fallback_struct_skip_field,
            backtrack_struct_skip_field: _,
            fallback_unit_variant,
            fallback_other_skip_item,
            backtrack_other_skip_item: _,
            allow_incomplete_string_in_key_or_variant: allow_incomplete_string_in_key,
        } = &mut self;

        *fallback_any_as_none = false;
        *fallback_ignored_any_as_none = false;
        *fallback_default_bool = None;
        *fallback_int_zero = false;
        *fallback_default_float = None;
        *fallback_default_char = None;
        *fallback_default_str = None;
        *fallback_bytes_empty = false;
        *fallback_none = false;
        *fallback_none_at_mandatory = false;
        *fallback_unit = false;
        *fallback_unit_at_mandatory = false;
        *fallback_unit_struct = false;
        *fallback_unit_struct_at_mandatory = false;
        *fallback_seq_empty = false;
        *fallback_seq_empty_at_root = false;
        *fallback_seq_skip_item = false;
        *fallback_tuple_empty = false;
        *fallback_tuple_skip_item = false;
        *fallback_tuple_struct_empty = false;
        *fallback_tuple_struct_skip_field = false;
        *fallback_map_empty = false;
        *fallback_map_empty_at_root = false;
        *fallback_map_skip_item = false;
        *fallback_struct_empty = false;
        *fallback_struct_empty_at_root = false;
        *fallback_struct_skip_field = false;
        *fallback_unit_variant = false;
        *fallback_other_skip_item = false;
        *allow_incomplete_string_in_key = false;

        self
    }

    /// Maximally strict behavior. Probably this library behaves like ordinary
    /// deserialization with this behavior.
    pub fn strict() -> Self {
        Self {
            tolerate_deserializer_fail_after_visit_success: false,
            fallback_any_as_none: false,
            fallback_ignored_any_as_none: false,
            fallback_default_bool: None,
            fallback_int_zero: false,
            fallback_default_float: None,
            fallback_default_char: None,
            fallback_default_str: None,
            fallback_bytes_empty: false,
            fallback_none: false,
            fallback_none_at_mandatory: false,
            fallback_unit: false,
            fallback_unit_at_mandatory: false,
            fallback_unit_struct: false,
            fallback_unit_struct_at_mandatory: false,
            fallback_seq_empty: false,
            fallback_seq_empty_at_root: false,
            backtrack_seq_empty_for_value: false,
            fallback_seq_skip_item: false,
            backtrack_seq_skip_item: false,
            fallback_tuple_empty: false,
            fallback_tuple_skip_item: false,
            backtrack_tuple_skip_item: false,
            fallback_tuple_struct_skip_item: false,
            backtrack_tuple_struct_skip_item: false,
            fallback_tuple_struct_empty: false,
            fallback_map_empty: false,
            fallback_map_empty_at_root: false,
            backtrack_map_empty_for_value: false,
            fallback_map_skip_item: false,
            backtrack_map_skip_item: false,
            fallback_struct_empty: false,
            fallback_struct_empty_at_root: false,
            backtrack_struct_empty_for_value: false,
            fallback_struct_skip_field: false,
            backtrack_struct_skip_field: false,
            fallback_unit_variant: false,
            fallback_other_skip_item: false,
            backtrack_other_skip_item: false,
            allow_incomplete_string_in_key_or_variant: false,
        }
    }

    pub fn lenient() -> Self {
        Self {
            tolerate_deserializer_fail_after_visit_success: true,
            fallback_any_as_none: true,
            fallback_ignored_any_as_none: true,
            fallback_default_bool: Some(false),
            fallback_int_zero: true,
            fallback_default_float: Some(0.0),
            fallback_default_char: Some('\0'),
            fallback_default_str: Some(""),
            fallback_bytes_empty: true,
            fallback_none: true,
            fallback_none_at_mandatory: true,
            fallback_unit: true,
            fallback_unit_at_mandatory: true,
            fallback_unit_struct: true,
            fallback_unit_struct_at_mandatory: true,
            fallback_seq_empty: true,
            fallback_seq_empty_at_root: true,
            backtrack_seq_empty_for_value: true,
            fallback_seq_skip_item: true,
            backtrack_seq_skip_item: true,
            fallback_tuple_empty: true,
            fallback_tuple_skip_item: true,
            backtrack_tuple_skip_item: true,
            fallback_tuple_struct_empty: true,
            fallback_tuple_struct_skip_item: true,
            backtrack_tuple_struct_skip_item: true,
            fallback_map_empty: true,
            fallback_map_empty_at_root: true,
            backtrack_map_empty_for_value: true,
            fallback_map_skip_item: true,
            backtrack_map_skip_item: true,
            fallback_struct_empty: true,
            fallback_struct_empty_at_root: true,
            backtrack_struct_empty_for_value: true,
            fallback_struct_skip_field: true,
            backtrack_struct_skip_field: true,
            fallback_unit_variant: true,
            fallback_other_skip_item: true,
            backtrack_other_skip_item: true,
            allow_incomplete_string_in_key_or_variant: true,
        }
    }
}

impl<Extra: ExtraOptions> Options<Extra> {
    /// Deserialize from a generic [`Source`].
    ///
    /// Unlike [`Self::deserialize_from_json_str`] etc, this method does not automatically append
    /// a random trailer. If you want that, then you can use
    /// [`Self::prepare_str_for_borrowed_deserialization`].
    ///
    /// You can use [`Options::deserialize_seed`] instead if you need to pass a seed.
    pub fn deserialize_source<'de, T, S>(self, source: S) -> Result<T, Error<S::Error>>
    where
        T: Deserialize<'de>,
        S: Source<'de>,
    {
        self.deserialize_seed(PhantomData, source)
    }

    /// Deserialize from a seed.
    ///
    /// Unlike [`Self::deserialize_from_json_str`] etc, this method does not automatically append
    /// a random trailer. If you want that, then you can use
    /// [`Self::prepare_str_for_borrowed_deserialization`].
    ///
    /// If you don't need a seed, then you can use [`Options::deserialize_source`].
    pub fn deserialize_seed<'de, T, S>(
        self,
        seed: T,
        mut source: S,
    ) -> Result<T::Value, Error<S::Error>>
    where
        T: DeserializeSeed<'de> + Clone,
        S: Source<'de>,
    {
        let mut state = self.build();
        let mut attempt = AttemptState::initial(&state);

        while {
            let max_n_backtracks = state.config.max_n_backtracks;
            max_n_backtracks
                .map(|max| state.n_backtracks <= max)
                .unwrap_or(true)
        } {
            let mut inner_deserializer_storage = Some(source.recreate_deserializer_storage());
            let inner_deserializer =
                S::use_deserializer_from_storage(&mut inner_deserializer_storage);

            let deserializer = crate::attempt::Deserializer {
                global: &mut state,
                attempt: &mut attempt,
                is_at_root: true,
                is_for_key_or_variant: false,
                is_for_map_value: false,
                inner: inner_deserializer,
            };

            match seed.clone().deserialize(deserializer) {
                Ok(value) => return Ok(value),
                Err(error) => {
                    debug!(attempt = state.n_backtracks, %error, "attempt failed");
                }
            }

            attempt = match attempt.next_attempt_state_after_failure()? {
                Some(new_attempt) => new_attempt,
                None => {
                    return Err(InternalError::NoPotentialBacktrackPoint {
                        after_backtracks: state.n_backtracks,
                    }
                    .into());
                }
            };
            state.n_backtracks += 1;
        }

        Err(InternalError::TooManyBacktracks.into())
    }
}
