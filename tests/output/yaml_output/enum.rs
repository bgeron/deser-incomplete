use serde::{Deserialize, Serialize};

use super::{default_modes, run_yaml_modes_on_prefixes_and_format_outputs};

#[test]
fn test_enum_external() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    enum External {
        Unit,
        Newtype(String),
        Tuple(String, #[serde(default)] String),
        Struct {
            a: String,
            #[serde(default)]
            b: String,
        },
    }

    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<External>>(&default_modes(), br#"["Unit", !Unit null, !Newtype "az", !Tuple ["az", "by"], !Struct {"a": "az", "b": "by"}]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[\"Unit\"": Ok([
          Unit,
        ]),
        "[\"Unit\", !Unit": Ok([
          Unit,
          Unit,
        ]),
        "[\"Unit\", !Unit null, !Newtype": Ok([
          Unit,
          Unit,
          Newtype(""),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"a": Ok([
          Unit,
          Unit,
          Newtype("a"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az": Ok([
          Unit,
          Unit,
          Newtype("az"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"]": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"b\": \"by\"}": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
          Struct(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[\"Unit\"": Ok([
          Unit,
        ]),
        "[\"Unit\", !Unit": Ok([
          Unit,
          Unit,
        ]),
        "[\"Unit\", !Unit null, !Newtype": Ok([
          Unit,
          Unit,
          Newtype(""),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"": Ok([]),
        "[\"Unit\", !Unit null, !Newtype \"az\"": Ok([
          Unit,
          Unit,
          Newtype("az"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"": Ok([]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\"": Ok([
          Unit,
          Unit,
          Newtype("az"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"": Ok([]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"": Ok([
          Unit,
          Unit,
          Newtype("az"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"]": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"": Ok([]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\"": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"": Ok([]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\"": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"": Ok([]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"b\"": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"b\": \"": Ok([]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"b\": \"by\"": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"b\": \"by\"}": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
          Struct(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[\"Unit\", !Unit n": Ok([
          Unit,
          Unit,
        ]),
        "[\"Unit\", !Unit null": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"b\": \"by\"}]": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
          Struct(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"b\": \"by\"}]": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
          Struct(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[\"Unit\"": Ok([
          Unit,
        ]),
        "[\"Unit\", !Unit": Ok([
          Unit,
          Unit,
        ]),
        "[\"Unit\", !Unit n": Ok([
          Unit,
        ]),
        "[\"Unit\", !Unit null": Ok([
          Unit,
          Unit,
        ]),
        "[\"Unit\", !Unit null, !Newtype": Ok([
          Unit,
          Unit,
          Newtype(""),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"a": Ok([
          Unit,
          Unit,
          Newtype("a"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az": Ok([
          Unit,
          Unit,
          Newtype("az"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"]": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"b\": \"by\"}": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
          Struct(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[\"Unit\"": Ok([
          Unit,
        ]),
        "[\"Unit\", !Unit": Ok([
          Unit,
          Unit,
        ]),
        "[\"Unit\", !Unit null, !Newtype": Ok([
          Unit,
          Unit,
          Newtype(""),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"a": Ok([
          Unit,
          Unit,
          Newtype("a"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az": Ok([
          Unit,
          Unit,
          Newtype("az"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"]": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
        ]),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"b\": \"by\"}": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
          Struct(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[\"Unit\", !Unit null, !Newtype \"az\", !Tuple [\"az\", \"by\"], !Struct {\"a\": \"az\", \"b\": \"by\"}]": Ok([
          Unit,
          Unit,
          Newtype("az"),
          Tuple("az", "by"),
          Struct(
            a: "az",
            b: "by",
          ),
        ]),
      },
    }
    "###);
}

#[test]
fn test_enum_internal() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    #[serde(tag = "type")]
    enum Internal {
        Unit,
        Newtype(Value),
        // Tuples are impossible with internally tagged enums
        Struct {
            a: String,
            #[serde(default)]
            b: String,
        },
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Value {
        value: String,
    }

    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<Internal>>(&default_modes(), br#"[{"type": "Unit"}, {"type": "Newtype", "value": "az", "extra": "junk"}, {"type": "Struct", "a": "az", "b": "by", "extra": "junk"}, {"type": "Unit"}]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[{\"type\": \"Unit": Ok([
          Internal(
            type: "Unit",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"a": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "a",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"a": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "a",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\":": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"b": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "b",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {\"type\": \"Unit": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
          Internal(
            type: "Unit",
          ),
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[{\"type\": \"Unit\"": Ok([
          Internal(
            type: "Unit",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\"": Ok([
          Internal(
            type: "Unit",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\"": Ok([
          Internal(
            type: "Unit",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\"": Ok([
          Internal(
            type: "Unit",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\":": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {\"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {\"type\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {\"type\": \"": Ok([]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {\"type\": \"Unit\"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
          Internal(
            type: "Unit",
          ),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {\"type\": \"Unit\"}]": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
          Internal(
            type: "Unit",
          ),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {\"type\": \"Unit\"}]": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
          Internal(
            type: "Unit",
          ),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit": Ok([
          Internal(
            type: "Unit",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"a": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "a",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"a": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "a",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\"": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\":": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"b": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "b",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {\"type\": \"Unit": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
          Internal(
            type: "Unit",
          ),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[{": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit": Ok([
          Internal(
            type: "Unit",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"a": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "a",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"a": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "a",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\"": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\":": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"b": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "b",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
        ]),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {\"type\": \"Unit": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
          Internal(
            type: "Unit",
          ),
        ]),
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[{\"type\": \"Unit\"}, {\"type\": \"Newtype\", \"value\": \"az\", \"extra\": \"junk\"}, {\"type\": \"Struct\", \"a\": \"az\", \"b\": \"by\", \"extra\": \"junk\"}, {\"type\": \"Unit\"}]": Ok([
          Internal(
            type: "Unit",
          ),
          Value(
            type: "Newtype",
            value: "az",
          ),
          Internal(
            type: "Struct",
            a: "az",
            b: "by",
          ),
          Internal(
            type: "Unit",
          ),
        ]),
      },
    }
    "###)
}

#[test]
fn test_enum_untagged() {
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    #[serde(untagged)]
    enum Untagged {
        Unit,
        Newtype(String),
        Tuple(String, #[serde(default)] String),
        Struct {
            a: String,
            #[serde(default)]
            b: String,
        },
    }

    insta::assert_ron_snapshot!(
        run_yaml_modes_on_prefixes_and_format_outputs::<Vec<Untagged>>(&default_modes(), br#"[null, "new", ["az", "by"], {"a": "az", "b": "by"}]"#),
        @r###"
    {
      "default behavior": {
        "": Ok([]),
        "[n": Ok([
          "n",
        ]),
        "[nu": Ok([
          "nu",
        ]),
        "[nul": Ok([
          "nul",
        ]),
        "[null": Ok([
          (),
        ]),
        "[null, \"": Ok([
          (),
          "",
        ]),
        "[null, \"n": Ok([
          (),
          "n",
        ]),
        "[null, \"ne": Ok([
          (),
          "ne",
        ]),
        "[null, \"new": Ok([
          (),
          "new",
        ]),
        "[null, \"new\", [\"": Ok([
          (),
          "new",
          ("", ""),
        ]),
        "[null, \"new\", [\"a": Ok([
          (),
          "new",
          ("a", ""),
        ]),
        "[null, \"new\", [\"az": Ok([
          (),
          "new",
          ("az", ""),
        ]),
        "[null, \"new\", [\"az\", \"b": Ok([
          (),
          "new",
          ("az", "b"),
        ]),
        "[null, \"new\", [\"az\", \"by": Ok([
          (),
          "new",
          ("az", "by"),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"a": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "a",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\"": Ok([
          (),
          "new",
          ("az", "by"),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\":": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"b": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "b",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"by": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "default behavior except no randomized trailer": {
        "": Ok([]),
        "[n": Ok([
          "n",
        ]),
        "[nu": Ok([
          "nu",
        ]),
        "[nul": Ok([
          "nul",
        ]),
        "[null": Ok([
          (),
        ]),
        "[null, \"": Ok([]),
        "[null, \"new\"": Ok([
          (),
          "new",
        ]),
        "[null, \"new\", [\"": Ok([]),
        "[null, \"new\", [\"az\"": Ok([
          (),
          "new",
          ("az", ""),
        ]),
        "[null, \"new\", [\"az\", \"": Ok([]),
        "[null, \"new\", [\"az\", \"by\"": Ok([
          (),
          "new",
          ("az", "by"),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"": Ok([]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\"": Ok([
          (),
          "new",
          ("az", "by"),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"": Ok([]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\"": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"": Ok([]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\"": Ok([
          (),
          "new",
          ("az", "by"),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\":": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"": Ok([]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"by\"": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "default behavior, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"by\"}]": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "no fallbacks, 0 backtracks": {
        "": Ok([]),
        "[": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"by\"}]": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "no fallbacks, 1 backtracks": {
        "": Ok([]),
        "[n": Ok([
          "n",
        ]),
        "[nu": Ok([
          "nu",
        ]),
        "[nul": Ok([
          "nul",
        ]),
        "[null": Ok([
          (),
        ]),
        "[null, \"": Ok([
          (),
          "",
        ]),
        "[null, \"n": Ok([
          (),
          "n",
        ]),
        "[null, \"ne": Ok([
          (),
          "ne",
        ]),
        "[null, \"new": Ok([
          (),
          "new",
        ]),
        "[null, \"new\", [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, \"new\", [\"": Ok([
          (),
          "new",
          ("", ""),
        ]),
        "[null, \"new\", [\"a": Ok([
          (),
          "new",
          ("a", ""),
        ]),
        "[null, \"new\", [\"az": Ok([
          (),
          "new",
          ("az", ""),
        ]),
        "[null, \"new\", [\"az\", \"b": Ok([
          (),
          "new",
          ("az", "b"),
        ]),
        "[null, \"new\", [\"az\", \"by": Ok([
          (),
          "new",
          ("az", "by"),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"a": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "a",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\"": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\":": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"b": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "b",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"by": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "default behavior, 1 backtracks": {
        "": Ok([]),
        "[n": Ok([
          "n",
        ]),
        "[nu": Ok([
          "nu",
        ]),
        "[nul": Ok([
          "nul",
        ]),
        "[null": Ok([
          (),
        ]),
        "[null, \"": Ok([
          (),
          "",
        ]),
        "[null, \"n": Ok([
          (),
          "n",
        ]),
        "[null, \"ne": Ok([
          (),
          "ne",
        ]),
        "[null, \"new": Ok([
          (),
          "new",
        ]),
        "[null, \"new\", [": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, \"new\", [\"": Ok([
          (),
          "new",
          ("", ""),
        ]),
        "[null, \"new\", [\"a": Ok([
          (),
          "new",
          ("a", ""),
        ]),
        "[null, \"new\", [\"az": Ok([
          (),
          "new",
          ("az", ""),
        ]),
        "[null, \"new\", [\"az\", \"b": Ok([
          (),
          "new",
          ("az", "b"),
        ]),
        "[null, \"new\", [\"az\", \"by": Ok([
          (),
          "new",
          ("az", "by"),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"a": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "a",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\"": Err("the maximum number of backtracks has been exceeded (see tracing logs for pointers to avoid a high number of backtracks)"),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\":": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"b": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "b",
          ),
        ]),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"by": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "by",
          ),
        ]),
      },
      "strict behavior": {
        "": Ok([]),
        "[": Err("could not find a potential backtrack point (do you have #[serde(default)] on your top-level type? are your settings too strict?) (after 0 backtracks)"),
        "[null, \"new\", [\"az\", \"by\"], {\"a\": \"az\", \"b\": \"by\"}]": Ok([
          (),
          "new",
          ("az", "by"),
          Untagged(
            a: "az",
            b: "by",
          ),
        ]),
      },
    }
    "###)
}
