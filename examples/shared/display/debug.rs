use futures::future::BoxFuture;
use futures::FutureExt;

use super::super::format::ParseOk;

use super::ActiveDisplay;

pub struct Display {
    pub prefix: &'static str,
}

impl ActiveDisplay for Display {
    fn descriptor(&self) -> Option<&str> {
        // It's kinda obvious that the output is debug output
        None
    }
    fn display_ok(&mut self, value: ParseOk) -> BoxFuture<'_, String> {
        let displayed = format!("{}{:?}", self.prefix, value);
        async { displayed }.boxed()
    }
}
