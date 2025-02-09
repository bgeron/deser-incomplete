use std::fmt::{Display, Formatter, Write as _};
use std::io::Write as _;

use super::{RandomTrailer, StringLike};

struct TagSuffix<'a> {
    add_backslash: bool,
    tag: &'a str,
}

impl Display for TagSuffix<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.add_backslash {
            f.write_str("\\ #")?;
        } else {
            f.write_str(" #\\\\")?;
        }
        write!(f, r#" # \"{}' # ""#, self.tag)
    }
}
const EXTRA_BYTES_IN_PARSED_SINGLE_QUOTED_STRING: usize = 9;
const TRAILER_FOR_DOUBLE_QUOTED_STRING: &str = "' # ";
const MINUS_BYTES_ON_TOP_FOR_DOUBLE_QUOTED_STRING: usize = 2;

#[derive(Clone, Debug, Default)]
pub struct YamlRandomTrailer;

impl RandomTrailer for YamlRandomTrailer {
    fn prepare_string_with_tag(&self, input: &mut String, tag: &str) {
        let add_backslash = heuristic_should_add_backslash(&**input);
        write!(input, "{}", TagSuffix { add_backslash, tag })
            .expect("writing to a string always succeeds")
    }

    fn prepare_vec_with_tag(&self, input: &mut Vec<u8>, tag: &str) {
        let add_backslash = heuristic_should_add_backslash(&**input);
        write!(input, "{}", TagSuffix { add_backslash, tag })
            .expect("writing to a vec always succeeds")
    }

    fn remove_trailer(&self, string_like: &mut impl StringLike, tag: &str) -> bool {
        if string_like.ends_with_string(tag) {
            // Single-quoted string
            let target_len =
                string_like.len() - tag.len() - EXTRA_BYTES_IN_PARSED_SINGLE_QUOTED_STRING;

            string_like.truncate_to_bytes(target_len);
            true
        } else if string_like.ends_with_2_strings(tag, TRAILER_FOR_DOUBLE_QUOTED_STRING) {
            // Double-quoted string
            let target_len = string_like.len()
                - tag.len()
                - (EXTRA_BYTES_IN_PARSED_SINGLE_QUOTED_STRING
                    - MINUS_BYTES_ON_TOP_FOR_DOUBLE_QUOTED_STRING)
                - TRAILER_FOR_DOUBLE_QUOTED_STRING.len();

            string_like.truncate_to_bytes(target_len);
            true
        } else {
            false
        }
    }
}

fn heuristic_should_add_backslash(s: impl StringLike) -> bool {
    n_trailing_backslashes(s) % 2 == 1
}

fn n_trailing_backslashes(mut s: impl StringLike) -> usize {
    let mut n = 0;
    while s.ends_with_string("\\") {
        n += 1;
        s.truncate_to_bytes(s.len() - 1);
    }
    n
}

#[cfg(test)]
#[path = ""]
mod test {

    #[path = "../../tests/output/yaml_output/cow_string.rs"]
    mod cow_string;

    use cow_string::CowString;
    use serde::Deserialize;

    use super::*;

    const TAG: &str = "BERLIN";

    fn parse_and_undo_tag(prepared: &str) -> (CowString<'_>, bool) {
        let mut parsed: CowString =
            CowString::deserialize(serde_yaml::Deserializer::from_str(prepared)).unwrap();

        let had_tag = match &mut parsed {
            CowString::VisitBorrowedStr(s) => YamlRandomTrailer.remove_trailer(s, TAG),
            CowString::VisitStr { cloned } => YamlRandomTrailer.remove_trailer(cloned, TAG),
            CowString::VisitString(s) => YamlRandomTrailer.remove_trailer(s, TAG),
        };

        (parsed, had_tag)
    }

    #[test]
    fn test_tag_suffix() {
        for (i, (input, will_encounter_end, expected_result)) in [
            (
                r#""Hello, wo"#,
                true,
                // CowString::VisitBorrowedStr("Hello, wo"),
                CowString::VisitStr {
                    cloned: "Hello, wo".to_string(),
                },
            ),
            (
                r#"'Hello, wo"#,
                true,
                CowString::VisitBorrowedStr("Hello, wo"),
            ),
            (
                r#""Hello, wo\"#,
                true,
                // Unfinished backslash escape in YAML -> we can't know what it'll eventually represent.
                //
                // serde_yaml has to convert "\t" into <TAB>, so the parsed YAML is no longer a substring
                // of the YAML itself -> CowString::Owned.
                CowString::VisitStr {
                    cloned: "Hello, wo".to_string(),
                },
            ),
            (
                r#"'Hello, wo\"#,
                true,
                // Unfinished backslash escape in YAML -> we can't know what it'll eventually represent.
                //
                // serde_yaml has to convert "\t" into <TAB>, so the parsed YAML is no longer a substring
                // of the YAML itself -> CowString::Owned.
                CowString::VisitBorrowedStr("Hello, wo"),
            ),
            (
                r#""Hello, world""#,
                false,
                CowString::VisitBorrowedStr("Hello, world"),
            ),
            (
                r#"'Hello, world'"#,
                false,
                CowString::VisitBorrowedStr("Hello, world"),
            ),
            (
                // \n converts into newline
                r#""Hello,\nworld""#,
                false,
                CowString::VisitStr {
                    cloned: "Hello,\nworld".to_string(),
                },
            ),
            (
                r#"'Hello,\nworld'"#,
                false,
                CowString::VisitBorrowedStr("Hello,\\nworld"),
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let mut prepared = input.to_string();
            YamlRandomTrailer.prepare_string_with_tag(&mut prepared, TAG);
            dbg!((i, &prepared));
            let (result, encountered_end): (CowString, bool) = parse_and_undo_tag(&prepared);
            assert_eq!(result, expected_result, "input = {input:?}");
            assert_eq!(encountered_end, will_encounter_end, "input = {input:?}");
            assert_eq!(
                matches!(result, CowString::VisitBorrowedStr(_)),
                matches!(expected_result, CowString::VisitBorrowedStr(_)),
                "input = {input:?}"
            )
        }
    }
}
