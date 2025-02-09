use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[test]
fn test_seq() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Vec<bool>>>(&default_modes(), &r#"[[], [true], [false], []]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[], [": Ok([
          [],
          [],
        ]),
        "[[], [true": Ok([
          [],
          [
            true,
          ],
        ]),
        "[[], [true], [": Ok([
          [],
          [
            true,
          ],
          [],
        ]),
        "[[], [true], [false": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
        ]),
        "[[], [true], [false], [": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
          [],
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[], [": Ok([
          [],
          [],
        ]),
        "[[], [true": Ok([
          [],
          [
            true,
          ],
        ]),
        "[[], [true], [": Ok([
          [],
          [
            true,
          ],
          [],
        ]),
        "[[], [true], [false": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
        ]),
        "[[], [true], [false], [": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
          [],
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[], [true], [false], []]": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
          [],
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[], [true], [false], []]": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
          [],
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[], [": Ok([
          [],
          [],
        ]),
        "[[], [true": Ok([
          [],
          [
            true,
          ],
        ]),
        "[[], [true], [": Ok([
          [],
          [
            true,
          ],
          [],
        ]),
        "[[], [true], [false": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
        ]),
        "[[], [true], [false], [": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
          [],
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[], [": Ok([
          [],
          [],
        ]),
        "[[], [true": Ok([
          [],
          [
            true,
          ],
        ]),
        "[[], [true], [": Ok([
          [],
          [
            true,
          ],
          [],
        ]),
        "[[], [true], [false": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
        ]),
        "[[], [true], [false], [": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
          [],
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[], [true], [false], []]": Ok([
          [],
          [
            true,
          ],
          [
            false,
          ],
          [],
        ]),
      },
    }
    "###
    );
}

#[test]
fn test_seq_cannot_parse_after_invalid() {
    // Note: the false is never reached because null is invalid there and serde_json
    // is unable to ever continue past that.
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Vec<bool>>>(&default_modes(), &r#"[[true], null, [true]]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[true": Ok([
          [
            true,
          ],
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[true": Ok([
          [
            true,
          ],
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "final output matches serde_json?": "serde_json failed",
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "final output matches serde_json?": "serde_json failed",
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[true": Ok([
          [
            true,
          ],
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[true": Ok([
          [
            true,
          ],
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "final output matches serde_json?": "serde_json failed",
      },
    }
    "###
    );
}

#[test]
fn test_seq_cannot_parse_after_trailing_comma() {
    // Note: the false is never reached because [true,] is invalid JSON and serde_json
    // is unable to ever continue past that.
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Vec<bool>>>(&default_modes(), &r#"[[true,], [false]]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[true": Ok([
          [
            true,
          ],
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[true": Ok([
          [
            true,
          ],
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "final output matches serde_json?": "serde_json failed",
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "final output matches serde_json?": "serde_json failed",
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[true": Ok([
          [
            true,
          ],
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[": Ok([
          [],
        ]),
        "[[true": Ok([
          [
            true,
          ],
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "final output matches serde_json?": "serde_json failed",
      },
    }
    "###
    );
}
