use serde::{Deserialize, Serialize};

use super::{default_modes, run_yaml_modes_on_prefixes_and_format_outputs};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct UnitStruct;

#[test]
fn test_toplevel_unit_struct() {
    insta::assert_ron_snapshot!(
            run_yaml_modes_on_prefixes_and_format_outputs::<UnitStruct>(&default_modes(), &"null"),
    @r###"
    {
      "default behavior": {
        "": Ok(UnitStruct),
      },
      "default behavior except no randomized trailer": {
        "": Ok(UnitStruct),
      },
      "default behavior, 0 backtracks": {
        "": Ok(UnitStruct),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok(UnitStruct),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(UnitStruct),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok(UnitStruct),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(UnitStruct),
      },
      "default behavior, 1 backtracks": {
        "": Ok(UnitStruct),
      },
      "strict behavior": {
        "": Ok(UnitStruct),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(UnitStruct),
      },
    }
    "###
     )
}

#[test]
fn test_toplevel_unit_struct_fail() {
    insta::assert_ron_snapshot!(
            run_yaml_modes_on_prefixes_and_format_outputs::<UnitStruct>(&default_modes(), &"notnull"),
    @r###"
    {
      "default behavior": {
        "": Ok(UnitStruct),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior except no randomized trailer": {
        "": Ok(UnitStruct),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 0 backtracks": {
        "": Ok(UnitStruct),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "no fallbacks, 0 backtracks": {
        "": Ok(UnitStruct),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "no fallbacks, 1 backtracks": {
        "": Ok(UnitStruct),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 1 backtracks": {
        "": Ok(UnitStruct),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "strict behavior": {
        "": Ok(UnitStruct),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
    }
    "###
     )
}

#[test]
fn test_unit_struct_fallible() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<UnitStruct>>(&default_modes(), &"[null, [], null]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[null": Ok([
          UnitStruct,
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[null": Ok([
          UnitStruct,
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
        "[null": Ok([
          UnitStruct,
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[null": Ok([
          UnitStruct,
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
    }
    "###
    );
}
