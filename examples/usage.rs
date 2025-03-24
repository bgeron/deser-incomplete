use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct S {
    a: String,
    b: Option<E>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
enum E {
    C { c: String },
    D(D),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct D {
    d: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .pretty()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Inspired by issue #3

    let _ = dbg!(deser_incomplete::from_json_str::<S>(
        r#"{"a": "abra", "b": {"C": {"c": "doh! "#
    ));
    let _ = dbg!(deser_incomplete::from_json_str::<S>(
        r#"{"a": "abra", "b": {"D": {"d": "doh! "#
    ));
}
