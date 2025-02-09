use super::{default_modes, run_yaml_modes_on_prefixes_and_format_outputs};

#[test]
fn test_tolerate_trailing_whitespace_like_serde_yaml() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<()>>(&default_modes(), &r#"[null]  "#),
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
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null]": Ok([
          (),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
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
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[null]": Ok([
          (),
        ]),
      },
    }
    "### )
}

#[test]
fn test_tolerate_trailing_junk_unlike_serde_yaml() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<()>>(&default_modes(), &r#"[null]  junk"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null]": Ok([
          (),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null]": Ok([
          (),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[null": Ok([
          (),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[null]": Ok([
          (),
        ]),
        "[null]  j": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
    }
    "### )
}

#[test]
fn test_toplevel_unit() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<()>(&default_modes(), &r#"null"#),
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
        "": Ok(()),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(()),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok(()),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(()),
      },
      "default behavior, 1 backtracks": {
        "": Ok(()),
      },
      "strict behavior": {
        "": Ok(()),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(()),
      },
    }
    "###)
}

#[test]
fn test_unit() {
    insta::assert_ron_snapshot!(
                run_yaml_modes_on_prefixes_and_format_outputs::<Vec<()>>(&default_modes(), &r#"[null, null]"#),
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
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, null]": Ok([
          (),
          (),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
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
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
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
        run_yaml_modes_on_prefixes_and_format_outputs::<Option<Vec<bool>>>(&default_modes(), &r#"null"#),
        
        
        @r###"
    {
      "default behavior": {
        "": Ok(None),
        "n": Ok(Some([])),
        "null": Ok(None),
      },
      "default behavior except no randomized trailer": {
        "": Ok(None),
        "n": Ok(Some([])),
        "null": Ok(None),
      },
      "default behavior, 0 backtracks": {
        "": Ok(None),
        "n": Ok(Some([])),
        "null": Ok(None),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok(None),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(None),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok(None),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(None),
      },
      "default behavior, 1 backtracks": {
        "": Ok(None),
        "n": Ok(Some([])),
        "null": Ok(None),
      },
      "strict behavior": {
        "": Ok(None),
        "n": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "null": Ok(None),
      },
    }
    "###)
}

#[test]
fn test_option() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<Option<Vec<bool>>>>(&default_modes(), &r#"[null, [], [true], [false], null]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
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
      "default behavior except no randomized trailer": {
        "": Ok([]),
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
      "default behavior, 0 backtracks": {
        "": Ok([]),
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
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
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
        "": Ok([]),
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
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
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
