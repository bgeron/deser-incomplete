#![cfg_attr(
    not(feature = "serde_json"),
    allow(unused_variables, unused_imports, dead_code)
)]

mod common;
#[cfg(feature = "serde_json")]
mod json_output;
#[cfg(feature = "serde_yaml")]
mod yaml_output;
