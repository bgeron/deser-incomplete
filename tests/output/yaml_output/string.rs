use super::{default_modes, run_yaml_modes_on_prefixes_and_format_outputs};

#[test]
fn test_string() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<String>>(&
            default_modes(), &r#"["abc", "de\nf"]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[\"": Ok([
          "",
        ]),
        "[\"a": Ok([
          "a",
        ]),
        "[\"ab": Ok([
          "ab",
        ]),
        "[\"abc": Ok([
          "abc",
        ]),
        "[\"abc\", \"": Ok([
          "abc",
          "",
        ]),
        "[\"abc\", \"d": Ok([
          "abc",
          "d",
        ]),
        "[\"abc\", \"de": Ok([
          "abc",
          "de",
        ]),
        "[\"abc\", \"de\\n": Ok([
          "abc",
          "de\n",
        ]),
        "[\"abc\", \"de\\nf": Ok([
          "abc",
          "de\nf",
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[\"abc\"": Ok([
          "abc",
        ]),
        "[\"abc\", \"": Ok([]),
        "[\"abc\", \"de\\nf\"": Ok([
          "abc",
          "de\nf",
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[\"abc\", \"de\\nf\"]": Ok([
          "abc",
          "de\nf",
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[\"abc\", \"de\\nf\"]": Ok([
          "abc",
          "de\nf",
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[\"": Ok([
          "",
        ]),
        "[\"a": Ok([
          "a",
        ]),
        "[\"ab": Ok([
          "ab",
        ]),
        "[\"abc": Ok([
          "abc",
        ]),
        "[\"abc\", \"": Ok([
          "abc",
          "",
        ]),
        "[\"abc\", \"d": Ok([
          "abc",
          "d",
        ]),
        "[\"abc\", \"de": Ok([
          "abc",
          "de",
        ]),
        "[\"abc\", \"de\\n": Ok([
          "abc",
          "de\n",
        ]),
        "[\"abc\", \"de\\nf": Ok([
          "abc",
          "de\nf",
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[\"": Ok([
          "",
        ]),
        "[\"a": Ok([
          "a",
        ]),
        "[\"ab": Ok([
          "ab",
        ]),
        "[\"abc": Ok([
          "abc",
        ]),
        "[\"abc\", \"": Ok([
          "abc",
          "",
        ]),
        "[\"abc\", \"d": Ok([
          "abc",
          "d",
        ]),
        "[\"abc\", \"de": Ok([
          "abc",
          "de",
        ]),
        "[\"abc\", \"de\\n": Ok([
          "abc",
          "de\n",
        ]),
        "[\"abc\", \"de\\nf": Ok([
          "abc",
          "de\nf",
        ]),
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[\"abc\", \"de\\nf\"]": Ok([
          "abc",
          "de\nf",
        ]),
      },
    }
    "###)
}
