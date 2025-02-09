use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[test]
fn test_bools() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<bool>>(&default_modes(), &"[true, false, true]"),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[true": Ok([
          true,
        ]),
        "[true, false": Ok([
          true,
          false,
        ]),
        "[true, false, true": Ok([
          true,
          false,
          true,
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[true": Ok([
          true,
        ]),
        "[true, false": Ok([
          true,
          false,
        ]),
        "[true, false, true": Ok([
          true,
          false,
          true,
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[true, false, true]": Ok([
          true,
          false,
          true,
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[true, false, true]": Ok([
          true,
          false,
          true,
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[true": Ok([
          true,
        ]),
        "[true, false": Ok([
          true,
          false,
        ]),
        "[true, false, true": Ok([
          true,
          false,
          true,
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[true": Ok([
          true,
        ]),
        "[true, false": Ok([
          true,
          false,
        ]),
        "[true, false, true": Ok([
          true,
          false,
          true,
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[true, false, true]": Ok([
          true,
          false,
          true,
        ]),
      },
    }
    "###
    );
}
