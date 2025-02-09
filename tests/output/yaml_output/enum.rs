use serde::{Deserialize, Serialize};

use super::{default_modes, run_yaml_modes_on_prefixes_and_format_outputs};

#[test]
fn test_unit_variant() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    enum Enum {
        UnitVariant,
    }

    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<Enum>>(&default_modes(), br#"["UnitVariant", "UnitVariant"]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[\"UnitVariant\"": Ok([
          UnitVariant,
        ]),
        "[\"UnitVariant\", \"UnitVariant\"": Ok([
          UnitVariant,
          UnitVariant,
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[\"UnitVariant\"": Ok([
          UnitVariant,
        ]),
        "[\"UnitVariant\", \"": Ok([]),
        "[\"UnitVariant\", \"UnitVariant\"": Ok([
          UnitVariant,
          UnitVariant,
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[\"UnitVariant\", \"UnitVariant\"]": Ok([
          UnitVariant,
          UnitVariant,
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[\"UnitVariant\", \"UnitVariant\"]": Ok([
          UnitVariant,
          UnitVariant,
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[\"UnitVariant\"": Ok([
          UnitVariant,
        ]),
        "[\"UnitVariant\", \"UnitVariant\"": Ok([
          UnitVariant,
          UnitVariant,
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[\"UnitVariant\"": Ok([
          UnitVariant,
        ]),
        "[\"UnitVariant\", \"UnitVariant\"": Ok([
          UnitVariant,
          UnitVariant,
        ]),
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[\"UnitVariant\", \"UnitVariant\"]": Ok([
          UnitVariant,
          UnitVariant,
        ]),
      },
    }
    "###);
}
