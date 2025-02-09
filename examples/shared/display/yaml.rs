use futures::future::BoxFuture;
use futures::FutureExt;

use super::super::format::ParseOk;

use super::ActiveDisplay;

pub struct Display;

impl ActiveDisplay for Display {
    fn descriptor(&self) -> Option<&str> {
        Some("YAML")
    }

    fn display_ok(&mut self, value: ParseOk) -> BoxFuture<'_, String> {
        let displayed = serde_yaml::to_string(&*value)
            .unwrap_or_else(|err| format!("could not convert back to YAML: {err}"));

        async { displayed }.boxed()
    }
}
