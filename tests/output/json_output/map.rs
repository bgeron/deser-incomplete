use indexmap::IndexMap;

use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[test]
fn test_toplevel_map() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<IndexMap<String, String>>(&
            default_modes(), &r#"{"abc": "def", "ghi": "jkl"}"#),
            @r###"
    {
      "default behavior": {
        "": Ok({}),
        "{\"abc\": \"": Ok({
          "abc": "",
        }),
        "{\"abc\": \"d": Ok({
          "abc": "d",
        }),
        "{\"abc\": \"de": Ok({
          "abc": "de",
        }),
        "{\"abc\": \"def": Ok({
          "abc": "def",
        }),
        "{\"abc\": \"def\", \"ghi\": \"": Ok({
          "abc": "def",
          "ghi": "",
        }),
        "{\"abc\": \"def\", \"ghi\": \"j": Ok({
          "abc": "def",
          "ghi": "j",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jk": Ok({
          "abc": "def",
          "ghi": "jk",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jkl": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "default behavior except no randomized trailer": {
        "": Ok({}),
        "{\"abc\": \"def\"": Ok({
          "abc": "def",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jkl\"": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "default behavior, 0 backtracks": {
        "": Ok({}),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"abc\": \"def\", \"ghi\": \"jkl\"}": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"abc\": \"def\", \"ghi\": \"jkl\"}": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Ok({}),
        "{\"abc\": \"": Ok({
          "abc": "",
        }),
        "{\"abc\": \"d": Ok({
          "abc": "d",
        }),
        "{\"abc\": \"de": Ok({
          "abc": "de",
        }),
        "{\"abc\": \"def": Ok({
          "abc": "def",
        }),
        "{\"abc\": \"def\", \"ghi\": \"": Ok({
          "abc": "def",
          "ghi": "",
        }),
        "{\"abc\": \"def\", \"ghi\": \"j": Ok({
          "abc": "def",
          "ghi": "j",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jk": Ok({
          "abc": "def",
          "ghi": "jk",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jkl": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "default behavior, 1 backtracks": {
        "": Ok({}),
        "{\"abc\": \"": Ok({
          "abc": "",
        }),
        "{\"abc\": \"d": Ok({
          "abc": "d",
        }),
        "{\"abc\": \"de": Ok({
          "abc": "de",
        }),
        "{\"abc\": \"def": Ok({
          "abc": "def",
        }),
        "{\"abc\": \"def\", \"ghi\": \"": Ok({
          "abc": "def",
          "ghi": "",
        }),
        "{\"abc\": \"def\", \"ghi\": \"j": Ok({
          "abc": "def",
          "ghi": "j",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jk": Ok({
          "abc": "def",
          "ghi": "jk",
        }),
        "{\"abc\": \"def\", \"ghi\": \"jkl": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"abc\": \"def\", \"ghi\": \"jkl\"}": Ok({
          "abc": "def",
          "ghi": "jkl",
        }),
      },
    }
    "###)
}

#[test]
fn test_map() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<IndexMap<String, String>>>(&
            default_modes(), &r#"[{"ab": "cd", "ef": "gh"}, {"AB": "CD", "EF": "GH"}]"#),
            @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[{": Ok([
          {},
        ]),
        "[{\"ab\": \"": Ok([
          {
            "ab": "",
          },
        ]),
        "[{\"ab\": \"c": Ok([
          {
            "ab": "c",
          },
        ]),
        "[{\"ab\": \"cd": Ok([
          {
            "ab": "cd",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"": Ok([
          {
            "ab": "cd",
            "ef": "",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"g": Ok([
          {
            "ab": "cd",
            "ef": "g",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {},
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"C": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "C",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"G": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "G",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"GH": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "GH",
          },
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[{": Ok([
          {},
        ]),
        "[{\"ab\": \"cd\"": Ok([
          {
            "ab": "cd",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {},
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\"": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"GH\"": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "GH",
          },
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"GH\"}]": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "GH",
          },
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"GH\"}]": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "GH",
          },
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[{": Ok([
          {},
        ]),
        "[{\"ab\": \"": Ok([
          {
            "ab": "",
          },
        ]),
        "[{\"ab\": \"c": Ok([
          {
            "ab": "c",
          },
        ]),
        "[{\"ab\": \"cd": Ok([
          {
            "ab": "cd",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"": Ok([
          {
            "ab": "cd",
            "ef": "",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"g": Ok([
          {
            "ab": "cd",
            "ef": "g",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {},
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"C": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "C",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"G": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "G",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"GH": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "GH",
          },
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[{": Ok([
          {},
        ]),
        "[{\"ab\": \"": Ok([
          {
            "ab": "",
          },
        ]),
        "[{\"ab\": \"c": Ok([
          {
            "ab": "c",
          },
        ]),
        "[{\"ab\": \"cd": Ok([
          {
            "ab": "cd",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"": Ok([
          {
            "ab": "cd",
            "ef": "",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"g": Ok([
          {
            "ab": "cd",
            "ef": "g",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {},
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"C": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "C",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"G": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "G",
          },
        ]),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"GH": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "GH",
          },
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[{\"ab\": \"cd\", \"ef\": \"gh\"}, {\"AB\": \"CD\", \"EF\": \"GH\"}]": Ok([
          {
            "ab": "cd",
            "ef": "gh",
          },
          {
            "AB": "CD",
            "EF": "GH",
          },
        ]),
      },
    }
    "###)
}
