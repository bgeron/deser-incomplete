use std::borrow::Cow;
use std::fmt::Debug;

use indexmap::IndexMap;

/// Run function on all prefixes of the input.
///
/// The output is deduplicated -- only inputs are shown where the output changes.
///
/// ### Panics
///
/// Out of precaution, when some output isn't equal to itself
pub(crate) fn run_on_prefixes_and_format_outputs<Output: Debug + PartialEq>(
    full_input: &[u8],
    f: impl Fn(&[u8]) -> Output,
) -> IndexMap<Cow<str>, Output> {
    let mut outputs: Vec<(&[u8], Output)> = Vec::new();

    for end in 0..=full_input.len() {
        let range = &full_input[..end];
        let output = f(range);

        #[allow(clippy::eq_op)]
        if output != output {
            panic!("value is not comparable to itself: {output:?}");
        }

        if Some(&output) != outputs.last().map(|(_, out)| out) {
            outputs.push((range, output));
        }
    }

    outputs
        .into_iter()
        .map(|(input, output)| (String::from_utf8_lossy(input), output))
        .collect()
}
