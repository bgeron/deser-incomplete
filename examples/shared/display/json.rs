use futures::future::BoxFuture;
use futures::FutureExt;

use super::super::format::ParseOk;

use super::ActiveDisplay;

pub struct Display {
    pub pretty: bool,
}

impl ActiveDisplay for Display {
    fn descriptor(&self) -> Option<&str> {
        Some("JSON")
    }

    fn display_ok(&mut self, value: ParseOk) -> BoxFuture<'_, String> {
        let displayed = (if self.pretty {
            serde_json::to_string_pretty(&*value)
        } else {
            serde_json::to_string(&*value)
        })
        .unwrap_or_else(|err| format!("could not convert back to JSON: {err}"));

        async { displayed }.boxed()
    }
}
