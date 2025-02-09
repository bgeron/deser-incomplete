use super::{default_modes, run_json_modes_on_prefixes_and_format_outputs};

#[test]
fn test_tolerate_trailing_whitespace_like_serde_json() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<()>>(&default_modes(), &r#"[null]  "#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null]": Ok([
          (),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null]": Ok([
          (),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[null": Ok([
          (),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[null]": Ok([
          (),
        ]),
      },
    }
    "### )
}

#[test]
fn test_tolerate_trailing_junk_unlike_serde_json() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<()>>(&default_modes(), &r#"[null]  junk"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null]": Ok([
          (),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null]": Ok([
          (),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[null": Ok([
          (),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[null]": Ok([
          (),
        ]),
        "final output matches serde_json?": "serde_json failed",
      },
    }
    "### )
}

#[test]
fn test_toplevel_unit() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<()>(&default_modes(), &r#"null"#),
        @r###"
    {
      "default behavior": {
        "": Ok(()),
      },
      "default behavior except no randomized trailer": {
        "": Ok(()),
      },
      "default behavior, 0 backtracks": {
        "": Ok(()),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(()),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(()),
      },
      "default behavior, 1 backtracks": {
        "": Ok(()),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(()),
      },
    }
    "###)
}

#[test]
fn test_unit() {
    insta::assert_ron_snapshot!(
                run_json_modes_on_prefixes_and_format_outputs::<Vec<()>>(&default_modes(), &r#"[null, null]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
        "[null, null": Ok([
          (),
          (),
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
        "[null, null": Ok([
          (),
          (),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, null]": Ok([
          (),
          (),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, null]": Ok([
          (),
          (),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[null": Ok([
          (),
        ]),
        "[null, null": Ok([
          (),
          (),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
        "[null, null": Ok([
          (),
          (),
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[null, null]": Ok([
          (),
          (),
        ]),
      },
    }
    "###
    );
}

#[test]
fn test_toplevel_none() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Option<Vec<bool>>>(&default_modes(), &r#"null"#),
        
        
        @r###"
    {
      "default behavior": {
        "": Ok(Some([])),
        "n": Ok(None),
      },
      "default behavior except no randomized trailer": {
        "": Ok(Some([])),
        "n": Ok(None),
      },
      "default behavior, 0 backtracks": {
        "": Ok(Some([])),
        "n": Ok(None),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(None),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(None),
      },
      "default behavior, 1 backtracks": {
        "": Ok(Some([])),
        "n": Ok(None),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(None),
      },
    }
    "###)
}

#[test]
fn test_option() {
    insta::assert_ron_snapshot!(
        run_json_modes_on_prefixes_and_format_outputs::<Vec<Option<Vec<bool>>>>(&default_modes(), &r#"[null, [], [true], [false], null]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[n": Ok([
          None,
        ]),
        "[null, [": Ok([
          None,
          Some([]),
        ]),
        "[null, [], [": Ok([
          None,
          Some([]),
          Some([]),
        ]),
        "[null, [], [true": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
        ]),
        "[null, [], [true], [": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([]),
        ]),
        "[null, [], [true], [false": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
        ]),
        "[null, [], [true], [false], n": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
          None,
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[n": Ok([
          None,
        ]),
        "[null, [": Ok([
          None,
          Some([]),
        ]),
        "[null, [], [": Ok([
          None,
          Some([]),
          Some([]),
        ]),
        "[null, [], [true": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
        ]),
        "[null, [], [true], [": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([]),
        ]),
        "[null, [], [true], [false": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
        ]),
        "[null, [], [true], [false], n": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
          None,
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[n": Ok([
          None,
        ]),
        "[null": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, [], [true], [false], n": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
          None,
        ]),
        "[null, [], [true], [false], null": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, [], [true], [false], null]": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
          None,
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, [], [true], [false], null]": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
          None,
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[": Ok([]),
        "[null": Ok([
          None,
        ]),
        "[null, [": Ok([
          None,
          Some([]),
        ]),
        "[null, [], [": Ok([
          None,
          Some([]),
          Some([]),
        ]),
        "[null, [], [true": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
        ]),
        "[null, [], [true], [": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([]),
        ]),
        "[null, [], [true], [false": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
        ]),
        "[null, [], [true], [false], null": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
          None,
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[n": Ok([
          None,
        ]),
        "[null, [": Ok([
          None,
          Some([]),
        ]),
        "[null, [], [": Ok([
          None,
          Some([]),
          Some([]),
        ]),
        "[null, [], [true": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
        ]),
        "[null, [], [true], [": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([]),
        ]),
        "[null, [], [true], [false": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
        ]),
        "[null, [], [true], [false], n": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
          None,
        ]),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[null, [], [true], [false], null]": Ok([
          None,
          Some([]),
          Some([
            true,
          ]),
          Some([
            false,
          ]),
          None,
        ]),
      },
    }
    "###
    );
}
