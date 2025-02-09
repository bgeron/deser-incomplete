use super::{default_modes, run_yaml_modes_on_prefixes_and_format_outputs};

#[test]
fn test_any() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<serde_yaml::Value>(&default_modes(), &"[true, false, 3, 4.5, \"abc\"]"),
        @r###"
    {
      "default behavior": {
        "": Ok(()),
        "[": Ok([]),
        "[t": Ok([
          "t",
        ]),
        "[tr": Ok([
          "tr",
        ]),
        "[tru": Ok([
          "tru",
        ]),
        "[true": Ok([
          true,
        ]),
        "[true, f": Ok([
          true,
          "f",
        ]),
        "[true, fa": Ok([
          true,
          "fa",
        ]),
        "[true, fal": Ok([
          true,
          "fal",
        ]),
        "[true, fals": Ok([
          true,
          "fals",
        ]),
        "[true, false": Ok([
          true,
          false,
        ]),
        "[true, false, 3": Ok([
          true,
          false,
          3,
        ]),
        "[true, false, 3, 4": Ok([
          true,
          false,
          3,
          4,
        ]),
        "[true, false, 3, 4.": Ok([
          true,
          false,
          3,
          4.0,
        ]),
        "[true, false, 3, 4.5": Ok([
          true,
          false,
          3,
          4.5,
        ]),
        "[true, false, 3, 4.5, \"": Ok([
          true,
          false,
          3,
          4.5,
          "",
        ]),
        "[true, false, 3, 4.5, \"a": Ok([
          true,
          false,
          3,
          4.5,
          "a",
        ]),
        "[true, false, 3, 4.5, \"ab": Ok([
          true,
          false,
          3,
          4.5,
          "ab",
        ]),
        "[true, false, 3, 4.5, \"abc": Ok([
          true,
          false,
          3,
          4.5,
          "abc",
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok(()),
        "[": Ok([]),
        "[t": Ok([
          "t",
        ]),
        "[tr": Ok([
          "tr",
        ]),
        "[tru": Ok([
          "tru",
        ]),
        "[true": Ok([
          true,
        ]),
        "[true, f": Ok([
          true,
          "f",
        ]),
        "[true, fa": Ok([
          true,
          "fa",
        ]),
        "[true, fal": Ok([
          true,
          "fal",
        ]),
        "[true, fals": Ok([
          true,
          "fals",
        ]),
        "[true, false": Ok([
          true,
          false,
        ]),
        "[true, false, 3": Ok([
          true,
          false,
          3,
        ]),
        "[true, false, 3, 4": Ok([
          true,
          false,
          3,
          4,
        ]),
        "[true, false, 3, 4.": Ok([
          true,
          false,
          3,
          4.0,
        ]),
        "[true, false, 3, 4.5": Ok([
          true,
          false,
          3,
          4.5,
        ]),
        "[true, false, 3, 4.5, \"": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[true, false, 3, 4.5, \"abc\"": Ok([
          true,
          false,
          3,
          4.5,
          "abc",
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok(()),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[true, false, 3, 4.5, \"abc\"]": Ok([
          true,
          false,
          3,
          4.5,
          "abc",
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok(()),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[true, false, 3, 4.5, \"abc\"]": Ok([
          true,
          false,
          3,
          4.5,
          "abc",
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok(()),
        "[": Ok([]),
        "[t": Ok([
          "t",
        ]),
        "[tr": Ok([
          "tr",
        ]),
        "[tru": Ok([
          "tru",
        ]),
        "[true": Ok([
          true,
        ]),
        "[true, f": Ok([
          true,
          "f",
        ]),
        "[true, fa": Ok([
          true,
          "fa",
        ]),
        "[true, fal": Ok([
          true,
          "fal",
        ]),
        "[true, fals": Ok([
          true,
          "fals",
        ]),
        "[true, false": Ok([
          true,
          false,
        ]),
        "[true, false, 3": Ok([
          true,
          false,
          3,
        ]),
        "[true, false, 3, 4": Ok([
          true,
          false,
          3,
          4,
        ]),
        "[true, false, 3, 4.": Ok([
          true,
          false,
          3,
          4.0,
        ]),
        "[true, false, 3, 4.5": Ok([
          true,
          false,
          3,
          4.5,
        ]),
        "[true, false, 3, 4.5, \"": Ok([
          true,
          false,
          3,
          4.5,
          "",
        ]),
        "[true, false, 3, 4.5, \"a": Ok([
          true,
          false,
          3,
          4.5,
          "a",
        ]),
        "[true, false, 3, 4.5, \"ab": Ok([
          true,
          false,
          3,
          4.5,
          "ab",
        ]),
        "[true, false, 3, 4.5, \"abc": Ok([
          true,
          false,
          3,
          4.5,
          "abc",
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok(()),
        "[": Ok([]),
        "[t": Ok([
          "t",
        ]),
        "[tr": Ok([
          "tr",
        ]),
        "[tru": Ok([
          "tru",
        ]),
        "[true": Ok([
          true,
        ]),
        "[true, f": Ok([
          true,
          "f",
        ]),
        "[true, fa": Ok([
          true,
          "fa",
        ]),
        "[true, fal": Ok([
          true,
          "fal",
        ]),
        "[true, fals": Ok([
          true,
          "fals",
        ]),
        "[true, false": Ok([
          true,
          false,
        ]),
        "[true, false, 3": Ok([
          true,
          false,
          3,
        ]),
        "[true, false, 3, 4": Ok([
          true,
          false,
          3,
          4,
        ]),
        "[true, false, 3, 4.": Ok([
          true,
          false,
          3,
          4.0,
        ]),
        "[true, false, 3, 4.5": Ok([
          true,
          false,
          3,
          4.5,
        ]),
        "[true, false, 3, 4.5, \"": Ok([
          true,
          false,
          3,
          4.5,
          "",
        ]),
        "[true, false, 3, 4.5, \"a": Ok([
          true,
          false,
          3,
          4.5,
          "a",
        ]),
        "[true, false, 3, 4.5, \"ab": Ok([
          true,
          false,
          3,
          4.5,
          "ab",
        ]),
        "[true, false, 3, 4.5, \"abc": Ok([
          true,
          false,
          3,
          4.5,
          "abc",
        ]),
      },
      "strict behavior": {
        "": Ok(()),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[true, false, 3, 4.5, \"abc\"]": Ok([
          true,
          false,
          3,
          4.5,
          "abc",
        ]),
      },
    }
    "###
    );
}
