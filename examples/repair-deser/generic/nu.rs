//! Utilities to call into nushell.

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

use crate::generic::format::ParseOk;

pub fn nu_version() -> Option<String> {
    let version = std::process::Command::new("nu")
        .args(["-c", "version | get version"])
        .output()
        .tap_err(|err| info!("failed to get nu version: {}", err))
        .ok()?
        .stdout
        .pipe(|bytes| String::from_utf8_lossy(&bytes).into_owned());

    // Versions start with a digit.
    if version
        .chars()
        .next()
        .is_some_and(|c| char::is_ascii_digit(&c))
    {
        Some(version.trim().to_string())
    } else {
        None
    }
}

/// Call into nushell, returning stdout as a stream of lines. May leak a process.
pub fn call_nu_returning_lines(
    command: String,
) -> anyhow::Result<(
    Arc<Mutex<impl Stream<Item = String> + Send + Unpin + 'static>>,
    tokio::process::ChildStdin,
)> {
    let (out_tx, out_rx) =
        interprocess::unnamed_pipe::pipe().context("could not construct pipe for output")?;
    let cmd = Command::new("nu")
        .args(["-c", &command])
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(OwnedFd::from(out_tx))
        .spawn()
        .context("could not spawn nushell to see if we have a compatible version")?;
    let outputs = out_rx
        .conv::<OwnedFd>()
        .conv::<std::process::ChildStderr>()
        .pipe(tokio::process::ChildStderr::from_std)
        .context("could not wrap nu stderr")?
        .pipe(Some)
        .unwrap()
        .pipe(BufReader::new)
        .lines()
        .pipe(LinesStream::new)
        .map(|out_json_result| {
            let out_json = match out_json_result {
                Ok(out_json) => out_json,
                Err(err) => return format!("could not read nushell table output: {err}"),
            };
            serde_json::from_str::<String>(&out_json)
                .unwrap_or_else(|err| format!("could not parse nushell table output: {err}"))
        })
        .pipe(Mutex::new)
        .pipe(Arc::new);
    Ok((outputs, cmd.stdin.unwrap()))
}
