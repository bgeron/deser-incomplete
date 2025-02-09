use serde::{Deserialize, Serialize};

use super::{default_modes, run_yaml_modes_on_prefixes_and_format_outputs};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Newtype(Vec<()>);

#[test]
fn test_toplevel_newtype_struct() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Newtype>(&default_modes(), &"[null, null]"),
        @r###"
    {
      "default behavior": {
        "": Ok(Newtype([])),
        "[null": Ok(Newtype([
          (),
        ])),
        "[null, null": Ok(Newtype([
          (),
          (),
        ])),
      },
      "default behavior except no randomized trailer": {
        "": Ok(Newtype([])),
        "[null": Ok(Newtype([
          (),
        ])),
        "[null, null": Ok(Newtype([
          (),
          (),
        ])),
      },
      "default behavior, 0 backtracks": {
        "": Ok(Newtype([])),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, null]": Ok(Newtype([
          (),
          (),
        ])),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok(Newtype([])),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, null]": Ok(Newtype([
          (),
          (),
        ])),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok(Newtype([])),
        "[null": Ok(Newtype([
          (),
        ])),
        "[null, null": Ok(Newtype([
          (),
          (),
        ])),
      },
      "default behavior, 1 backtracks": {
        "": Ok(Newtype([])),
        "[null": Ok(Newtype([
          (),
        ])),
        "[null, null": Ok(Newtype([
          (),
          (),
        ])),
      },
      "strict behavior": {
        "": Ok(Newtype([])),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[null, null]": Ok(Newtype([
          (),
          (),
        ])),
      },
    }
    "###)
}

#[test]
fn test_newtype_struct() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<Newtype>>(&default_modes(), &"[[], [null, null], []]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [null": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null, null": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
        ]),
        "[[], [null, null], [": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [null": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null, null": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
        ]),
        "[[], [null, null], [": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[], [null, null], []]": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[], [null, null], []]": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [null": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null, null": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
        ]),
        "[[], [null, null], [": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [null": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null, null": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
        ]),
        "[[], [null, null], [": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[], [null, null], []]": Ok([
          Newtype([]),
          Newtype([
            (),
            (),
          ]),
          Newtype([]),
        ]),
      },
    }
    "###)
}

#[test]
fn test_newtype_struct_fail() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<Newtype>>(&default_modes(), &"[[], [null null"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [null": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null n": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [null": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null n": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [null": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null n": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[": Ok([
          Newtype([]),
        ]),
        "[[], [": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "[[], [null": Ok([
          Newtype([]),
          Newtype([
            (),
          ]),
        ]),
        "[[], [null n": Ok([
          Newtype([]),
          Newtype([]),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
    }
    "###)
}
