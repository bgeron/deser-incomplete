use std::os::fd::OwnedFd;
use std::process::Stdio;
use std::sync::Arc;

use anyhow::Context as _;
use futures::future::BoxFuture;
use futures::{FutureExt, Stream};
use tap::{Conv, Pipe, TapFallible};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;
use tracing::{error, info, warn};

use super::super::format::ParseOk;
use super::super::nu::{call_nu_returning_stderr_lines, nu_version, KNOWN_GOOD_NU_VERSION};

use super::ActiveDisplay;

#[derive(Copy, Clone, Debug)]
pub struct Prefs {
    enable_color: bool,
    theme_name: &'static str,
}

impl Prefs {
    pub(crate) fn bare() -> Self {
        Self {
            enable_color: false,
            theme_name: "light-theme",
        }
    }

    pub(crate) fn light() -> Self {
        Self {
            enable_color: true,
            theme_name: "light-theme",
        }
    }

    pub(crate) fn dark() -> Self {
        Self {
            enable_color: true,
            theme_name: "dark-theme",
        }
    }
}

fn nu_command(prefs: Prefs) -> String {
    format!(
        // Double {{ to escape for format!() .
        //
        // We have to connect stdout to a terminal and let nushell write to stderr. This
        // way, we trigger `table`'s behavior to print nice colors.
        "use std/config {theme_name}; $env.config.color_config = ({theme_name}); open -r /dev/stdin | lines | each {{ from json | table -e -t default {strip_ansi} | to json -r }} | to text o> /dev/stderr",
        strip_ansi = if prefs.enable_color { "" } else { " | ansi strip" },
        theme_name = prefs.theme_name
    )
}

/// Format values using nushell.
///
/// This implementation may be a bit horrible, threads-wise.
pub struct Display {
    pub tableize: Tableize,
}

impl Display {
    pub async fn new_if_nu_installed(prefs: Prefs) -> Option<Self> {
        let version = nu_version()?;

        if check_nu_support().await != Some(true) {
            return None;
        }

        if version != KNOWN_GOOD_NU_VERSION {
            warn!("Using nu version {version} (known good version = {KNOWN_GOOD_NU_VERSION})")
        };

        Self::new_inner(prefs)
    }

    pub fn new_always(prefs: Prefs) -> Self {
        Self::new_inner(prefs).expect("could not start nushell displayer")
    }

    fn new_inner(prefs: Prefs) -> Option<Display> {
        let tableize = tableize_json_with_nu(prefs)
            .tap_err(|err| error!("could not start nushell displayer: {err}"))
            .ok()?;

        Some(Self { tableize })
    }
}

/// Check whether our version of nushell supports the command we need.
///
/// This may leak a process.
pub async fn check_nu_support() -> Option<bool> {
    let tableized = tableize_json_with_nu(Prefs::bare()).ok()?("'hello\"world'".to_string()).await;
    tableized.ok().map(|t| t == r#"'hello"world'"#)
}

pub type Tableize = Box<dyn FnMut(String) -> BoxFuture<'static, Result<String, String>> + Send>;

/// Use nushell to convert JSONs into tables.
///
/// May leak a process.
pub fn tableize_json_with_nu(prefs: Prefs) -> anyhow::Result<Tableize> {
    let command = nu_command(prefs);

    let (stdin, outputs) = call_nu_returning_stderr_lines(command)?;
    let stdin = Arc::new(Mutex::new(stdin));
    let outputs = Arc::new(Mutex::new(outputs));

    let f = move |input: String| {
        let stdin = stdin.clone();
        let outputs = outputs.clone();
        async move {
            let mut stdin_guard = stdin.lock().await;
            stdin_guard.write_all(input.as_bytes()).await.unwrap();
            stdin_guard.write_all(b"\n").await.unwrap();
            stdin_guard.flush().await.unwrap();

            outputs
                .try_lock()
                .expect("tried to tableize multiple values concurrently")
                .next()
                .await
                .unwrap_or_else(|| Err("nushell did not format our value".to_string()))
        }
        .boxed()
    };
    Ok(Box::new(f))
}

impl ActiveDisplay for Display {
    fn descriptor(&self) -> Option<&str> {
        Some("Formatting by nushell")
    }

    fn display_ok(&mut self, value: ParseOk) -> BoxFuture<'_, String> {
        async move {
            match (self.tableize)(serde_json::to_string(&*value).unwrap()).await {
                Ok(table) => table,
                Err(error) => error,
            }
        }
        .boxed()
    }
}
