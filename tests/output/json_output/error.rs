use serde::{Deserialize, Serialize};

use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[test]
fn test_top_level_struct() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct NeedsAllFields {
        bool: bool,
        int: u32,
        unit: (),
    }

    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<NeedsAllFields>(&default_modes(), &r#"{"bool": true, "int": 42, "unit": null}"# ),
        @r###"
    {
      "default behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"bool\": true, \"int\": 42, \"unit\":": Ok(NeedsAllFields(
          bool: true,
          int: 42,
          unit: (),
        )),
      },
      "default behavior except no randomized trailer": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"bool\": true, \"int\": 42, \"unit\":": Ok(NeedsAllFields(
          bool: true,
          int: 42,
          unit: (),
        )),
      },
      "default behavior, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"bool\": true, \"int\": 42, \"unit\":": Ok(NeedsAllFields(
          bool: true,
          int: 42,
          unit: (),
        )),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"bool\": true, \"int\": 42, \"unit\": null}": Ok(NeedsAllFields(
          bool: true,
          int: 42,
          unit: (),
        )),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"bool\": true, \"int\": 42, \"unit\": null": Ok(NeedsAllFields(
          bool: true,
          int: 42,
          unit: (),
        )),
      },
      "default behavior, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"bool\": true, \"int\": 42, \"unit\":": Ok(NeedsAllFields(
          bool: true,
          int: 42,
          unit: (),
        )),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"bool\": true, \"int\": 42, \"unit\": null}": Ok(NeedsAllFields(
          bool: true,
          int: 42,
          unit: (),
        )),
      },
    }
    "###)
}
