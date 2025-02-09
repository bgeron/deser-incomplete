use serde::{Deserialize, Serialize};

use super::{default_modes, run_yaml_modes_on_prefixes_and_format_outputs};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Struct {
    x: Vec<bool>,
    y: Vec<bool>,
    z: Vec<bool>,
}

#[test]
fn test_toplevel_struct() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Struct>(&default_modes(), &r#"{"x": [true], "y": [false], "z": [true]}"#),
        @r###"
    {
      "default behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\":": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "{\"x\": [": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 3 backtracks)"),
        "{\"x\": [true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\":": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "{\"x\": [true], \"y\": [": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 3 backtracks)"),
        "{\"x\": [true], \"y\": [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\"": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior except no randomized trailer": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\"": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\":": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "{\"x\": [": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 3 backtracks)"),
        "{\"x\": [true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\": [true], \"y\"": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\":": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 2 backtracks)"),
        "{\"x\": [true], \"y\": [": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 3 backtracks)"),
        "{\"x\": [true], \"y\": [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\"": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\"": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [true]": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [true]}": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\"": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false]": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\"": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [true]}": Ok(Struct(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
    }
    "###)
}

#[test]
fn test_struct() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<Struct>>(&default_modes(), &r#"[{"x": [true], "y": [false], "z": [true]}, {"x": [false], "y": [true], "z": [false]}]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}]": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}]": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\"": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}]": Ok([
          Struct(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          Struct(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
      },
    }
    "###)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct StructWithSomeDefaults {
    x: Vec<bool>,
    #[serde(default)]
    y: Vec<bool>,
    #[serde(default)]
    z: Vec<bool>,
}

#[test]
fn test_toplevel_struct_with_defaults() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<StructWithSomeDefaults>(&default_modes(), &r#"{"x": [true], "y": [false], "z": [true]}"#),
        @r###"
    {
      "default behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\"": Ok(StructWithSomeDefaults(
          x: [],
          y: [],
          z: [],
        )),
        "{\"x\": [true": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior except no randomized trailer": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\"": Ok(StructWithSomeDefaults(
          x: [],
          y: [],
          z: [],
        )),
        "{\"x\": [true": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\": [true], \"y\"": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\"": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\"": Ok(StructWithSomeDefaults(
          x: [],
          y: [],
          z: [],
        )),
        "{\"x\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true]": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"y\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false]": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [true]": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "no fallbacks, 0 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [true]}": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "no fallbacks, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\"": Ok(StructWithSomeDefaults(
          x: [],
          y: [],
          z: [],
        )),
        "{\"x\": [true": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "default behavior, 1 backtracks": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 1 backtracks)"),
        "{\"x\"": Ok(StructWithSomeDefaults(
          x: [],
          y: [],
          z: [],
        )),
        "{\"x\": [true": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [],
        )),
        "{\"x\": [true], \"y\": [false], \"z\": [true": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
      "strict behavior": {
        "": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\": [true], \"y\": [false], \"z\": [true]}": Ok(StructWithSomeDefaults(
          x: [
            true,
          ],
          y: [
            false,
          ],
          z: [
            true,
          ],
        )),
      },
    }
    "###)
}

#[test]
fn test_struct_with_defaults() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<StructWithSomeDefaults>>(&default_modes(), &r#"[{"x": [true], "y": [false], "z": [true]}, {"x": [false], "y": [true], "z": [false]}])"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[{\"x\"": Ok([
          StructWithSomeDefaults(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\"": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[{\"x\"": Ok([
          StructWithSomeDefaults(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"": Ok([]),
        "[{\"x\": [true], \"y\"": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\"": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\"": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\"": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"": Ok([]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\"": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\"": Ok([
          StructWithSomeDefaults(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true]": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false]": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\"": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false]": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true]": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}]": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}]": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\"": Ok([
          StructWithSomeDefaults(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\"": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\"": Ok([
          StructWithSomeDefaults(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\"": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}]": Ok([
          StructWithSomeDefaults(
            x: [
              true,
            ],
            y: [
              false,
            ],
            z: [
              true,
            ],
          ),
          StructWithSomeDefaults(
            x: [
              false,
            ],
            y: [
              true,
            ],
            z: [
              false,
            ],
          ),
        ]),
        "[{\"x\": [true], \"y\": [false], \"z\": [true]}, {\"x\": [false], \"y\": [true], \"z\": [false]}])": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
    }
    "###)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct NeedsNoFields {
    #[serde(default)]
    x: Vec<bool>,
}

#[test]
fn test_toplevel_struct_with_all_defaults() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<NeedsNoFields>(&default_modes(), &r#"{"x": [true]}"#),
        @r###"
    {
      "default behavior": {
        "": Ok(NeedsNoFields(
          x: [],
        )),
        "{\"x\": [true": Ok(NeedsNoFields(
          x: [
            true,
          ],
        )),
      },
      "default behavior except no randomized trailer": {
        "": Ok(NeedsNoFields(
          x: [],
        )),
        "{\"x\": [true": Ok(NeedsNoFields(
          x: [
            true,
          ],
        )),
      },
      "default behavior, 0 backtracks": {
        "": Ok(NeedsNoFields(
          x: [],
        )),
        "{\"x\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true]": Ok(NeedsNoFields(
          x: [
            true,
          ],
        )),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok(NeedsNoFields(
          x: [],
        )),
        "{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "{\"x\": [true]}": Ok(NeedsNoFields(
          x: [
            true,
          ],
        )),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok(NeedsNoFields(
          x: [],
        )),
        "{\"x\": [true": Ok(NeedsNoFields(
          x: [
            true,
          ],
        )),
      },
      "default behavior, 1 backtracks": {
        "": Ok(NeedsNoFields(
          x: [],
        )),
        "{\"x\": [true": Ok(NeedsNoFields(
          x: [
            true,
          ],
        )),
      },
      "strict behavior": {
        "": Ok(NeedsNoFields(
          x: [],
        )),
        "{": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "{\"x\": [true]}": Ok(NeedsNoFields(
          x: [
            true,
          ],
        )),
      },
    }
    "###)
}

#[test]
fn test_struct_with_all_defaults() {
    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<NeedsNoFields>>(&default_modes(), &r#"[{"x": [true]}, {"x": [false]}"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[{": Ok([
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true]}, {": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\": [true]}, {\"x\": [false": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [
              false,
            ],
          ),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[{": Ok([
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"": Ok([]),
        "[{\"x\"": Ok([
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true]}, {": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\": [true]}, {\"": Ok([]),
        "[{\"x\": [true]}, {\"x\"": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\": [true]}, {\"x\": [false": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [
              false,
            ],
          ),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{": Ok([
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true]": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true]}": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true]}, {": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\": [true]}, {\"x\":": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"x\": [true]}, {\"x\": [false]": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [
              false,
            ],
          ),
        ]),
        "[{\"x\": [true]}, {\"x\": [false]}": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[{": Ok([
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true]}, {": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\": [true]}, {\"x\": [false": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [
              false,
            ],
          ),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[{": Ok([
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\": [true": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
        ]),
        "[{\"x\": [true]}, {": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [],
          ),
        ]),
        "[{\"x\": [true]}, {\"x\": [false": Ok([
          NeedsNoFields(
            x: [
              true,
            ],
          ),
          NeedsNoFields(
            x: [
              false,
            ],
          ),
        ]),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "final output matches serde_yaml?": "serde_yaml failed",
      },
    }
    "###)
}
