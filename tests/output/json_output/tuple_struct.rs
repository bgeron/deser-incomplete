use serde::{Deserialize, Serialize};

use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Tuple(Vec<bool>, Vec<bool>, Vec<bool>);

#[test]
fn test_toplevel_tuple_struct() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Tuple>(&default_modes(), &"[[true], [false], [true]]"),
        @r###"
    {
      "default behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "[[true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[true], [": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "[[true], [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[true], [false], [": Ok(Tuple([
          true,
        ], [
          false,
        ], [])),
        "[[true], [false], [true": Ok(Tuple([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "default behavior except no randomized trailer": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "[[true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[true], [": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "[[true], [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[true], [false], [": Ok(Tuple([
          true,
        ], [
          false,
        ], [])),
        "[[true], [false], [true": Ok(Tuple([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "default behavior, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[true], [false], [true]": Ok(Tuple([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[true], [false], [true]": Ok(Tuple([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[true], [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[true], [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[true], [false], [": Ok(Tuple([
          true,
        ], [
          false,
        ], [])),
        "[[true], [false], [true": Ok(Tuple([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "default behavior, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[true], [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[true], [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[true], [false], [": Ok(Tuple([
          true,
        ], [
          false,
        ], [])),
        "[[true], [false], [true": Ok(Tuple([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[true], [false], [true]]": Ok(Tuple([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
    }
    "###)
}

#[test]
fn test_tuple_struct() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Tuple>>(&default_modes(), &"[[[true], [false], [true]], [[false], [true], [false]]]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[[true], [false], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[[[true], [false], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]]]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]]]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]]]": Ok([
          Tuple([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          Tuple([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
    }
    "###)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct TupleWithDefault(
    Vec<bool>,
    #[serde(default)] Vec<bool>,
    #[serde(default)] Vec<bool>,
);

#[test]
fn test_toplevel_tuple_struct_with_default() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<TupleWithDefault>(&default_modes(), &"[[true], [false], [true]]"),
        @r###"
    {
      "default behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[": Ok(TupleWithDefault([], [], [])),
        "[[true": Ok(TupleWithDefault([
          true,
        ], [], [])),
        "[[true], [false": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [])),
        "[[true], [false], [true": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "default behavior except no randomized trailer": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[": Ok(TupleWithDefault([], [], [])),
        "[[true": Ok(TupleWithDefault([
          true,
        ], [], [])),
        "[[true], [false": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [])),
        "[[true], [false], [true": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "default behavior, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[true], [false], [true]": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[true], [false], [true]": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[": Ok(TupleWithDefault([], [], [])),
        "[[true": Ok(TupleWithDefault([
          true,
        ], [], [])),
        "[[true], [false": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [])),
        "[[true], [false], [true": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "default behavior, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "[[": Ok(TupleWithDefault([], [], [])),
        "[[true": Ok(TupleWithDefault([
          true,
        ], [], [])),
        "[[true], [false": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [])),
        "[[true], [false], [true": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[true], [false], [true]]": Ok(TupleWithDefault([
          true,
        ], [
          false,
        ], [
          true,
        ])),
      },
    }
    "###)
}

#[test]
fn test_tuple_struct_with_default() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<TupleWithDefault>>(&default_modes(), &"[[[true], [false], [true]], [[false], [true], [false]]]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[[[": Ok([
          TupleWithDefault([], [], []),
        ]),
        "[[[true": Ok([
          TupleWithDefault([
            true,
          ], [], []),
        ]),
        "[[[true], [false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [[": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([], [], []),
        ]),
        "[[[true], [false], [true]], [[false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [], []),
        ]),
        "[[[true], [false], [true]], [[false], [true": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[[[": Ok([
          TupleWithDefault([], [], []),
        ]),
        "[[[true": Ok([
          TupleWithDefault([
            true,
          ], [], []),
        ]),
        "[[[true], [false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [[": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([], [], []),
        ]),
        "[[[true], [false], [true]], [[false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [], []),
        ]),
        "[[[true], [false], [true]], [[false], [true": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]]]": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false]]": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]]]": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[": Ok([
          TupleWithDefault([], [], []),
        ]),
        "[[[true": Ok([
          TupleWithDefault([
            true,
          ], [], []),
        ]),
        "[[[true], [false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([], [], []),
        ]),
        "[[[true], [false], [true]], [[false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [], []),
        ]),
        "[[[true], [false], [true]], [[false], [true": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[": Ok([
          TupleWithDefault([], [], []),
        ]),
        "[[[true": Ok([
          TupleWithDefault([
            true,
          ], [], []),
        ]),
        "[[[true], [false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], []),
        ]),
        "[[[true], [false], [true": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
        ]),
        "[[[true], [false], [true]], [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[[[true], [false], [true]], [[": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([], [], []),
        ]),
        "[[[true], [false], [true]], [[false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [], []),
        ]),
        "[[[true], [false], [true]], [[false], [true": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], []),
        ]),
        "[[[true], [false], [true]], [[false], [true], [false": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[[[true], [false], [true]], [[false], [true], [false]]]": Ok([
          TupleWithDefault([
            true,
          ], [
            false,
          ], [
            true,
          ]),
          TupleWithDefault([
            false,
          ], [
            true,
          ], [
            false,
          ]),
        ]),
      },
    }
    "###)
}
