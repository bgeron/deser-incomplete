#![allow(unused_imports)]

use std::cell::RefCell;
use std::io::{Read, Write};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use clap::{ArgAction, Parser, ValueEnum};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use shared::display::DisplayPreference;
use shared::nu::{call_nu_returning_stderr_lines, nu_version, KNOWN_GOOD_NU_VERSION};
use tokio::task::spawn_blocking;
use tokio_stream::StreamExt as _;
use tracing::level_filters::LevelFilter;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[allow(dead_code)]
#[path = "shared/mod.rs"]
mod shared;

/// Get disk usage for files (default: /*), and print as a streaming JSON list.
#[derive(Parser, Debug, Clone)]
#[command(version)]
struct Args {
    /// The pattern of directories/files to count disk usage for.
    ///
    /// Warning! These strings is fed directly to shell without any escaping.
    #[clap(default_value = "/*")]
    patterns: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .pretty()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    match nu_version() {
        None => panic!("nushell not found on this system. Consider running: cargo install --locked nu@{KNOWN_GOOD_NU_VERSION}"),
        Some(version) => {
            if version != KNOWN_GOOD_NU_VERSION {
                info!("Found nushell {version} (version {KNOWN_GOOD_NU_VERSION} is known to work well with deser-incomplete examples)");
            }
        }
    }

    let du_result_serializable = du(args.patterns.join(" "));

    spawn_blocking(move || {
        serde_json::to_writer(std::io::stdout(), &du_result_serializable)
            .expect("could not serialize")
    })
    .await
    .unwrap();

    Ok(())
}

/// Return du results, which will be serializable exactly once, and
/// serialization will be blocking.
///
/// We make no attempt to escape the pattern as required. The wrong=
/// argument may kill your computer.
///
/// This flushes stdout liberally.
///
/// Example argument: "/*"
fn du(pattern: String) -> impl Serialize + Send + Sync + 'static {
    let (_stdin, mut out_stream) = call_nu_returning_stderr_lines(format!(
        "ls -D {pattern}{rest}",
        rest = "
            | where type in [file dir]
            | each { |row|
                # Create one closure for the name, and one for
                # the du result. This way we can output them separately.
                [
                    { $row | select name },
                    { du $row.name
                      | reject path
                      | update apparent { to text }
                      | update physical { to text }
                      | get 0 }
                ] }
            | flatten
            | each { do $in | to json -r | to json -r }
            | to text o> /dev/stderr
            "
    ))
    .expect("could not call nushell");

    // Convert the stream into an iterator. (The iterator MUST NOT be accessed
    // in async code.)
    let (tx, rx) = std::sync::mpsc::channel::<FieldGroup>();

    tokio::spawn(async move {
        while let Some(line) = out_stream.next().await {
            let line = line.expect("could not read line from nushell");
            let parsed = serde_json::from_str(&line).unwrap_or_else(|err| {
                panic!("result line from nushell did not parse ({err}): {line:?}")
            });
            if tx.send(parsed).is_err() {
                // channel closed
                break;
            }
        }
    });

    // Each row will consist of two field groups.
    let field_groups = Arc::new(Mutex::new(
        std::iter::from_fn(move || -> Option<FieldGroup> { rx.recv().ok() }).fuse(),
    ));

    let rows = std::iter::from_fn(move || {
        // Compute a lazy iterator for fields.
        std::io::stdout().flush().expect("could not flush stdout");

        let first_field_group = field_groups.lock().expect("poison").next()?;

        let field_groups = field_groups.clone();
        let row = first_field_group
            .into_iter()
            // We now have the fields of the first field group. Append fields
            // of the second field group, if there will be one.
            .chain(
                std::iter::once_with(move || {
                    std::io::stdout().flush().expect("could not flush stdout");
                    field_groups.lock().expect("poison").next()
                })
                .flatten() // remove option
                .flatten(), // iterate over fields
            );
        Some(SerializeMapOnce::new(row))
    })
    .fuse();

    SerializeSeqOnce::new(rows)
}

type FieldGroup = IndexMap<String, serde_json::Value>;

struct SerializeSeqOnce<I>(Mutex<Option<I>>);

impl<I> SerializeSeqOnce<I> {
    fn new(iter: I) -> Self {
        Self(Mutex::new(Some(iter)))
    }
}

struct SerializeMapOnce<I>(Mutex<Option<I>>);

impl<I> SerializeMapOnce<I> {
    fn new(iter: I) -> Self {
        Self(Mutex::new(Some(iter)))
    }
}

impl<I> Serialize for SerializeSeqOnce<I>
where
    I: Iterator,
    I::Item: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.collect_seq(
            self.0
                .lock()
                .expect("poison")
                .take()
                .expect("tried to serialize from the iterator twice"),
        )
    }
}

impl<I, K, V> Serialize for SerializeMapOnce<I>
where
    I: Iterator<Item = (K, V)>,
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.collect_map(
            self.0
                .lock()
                .expect("poison")
                .take()
                .expect("tried to serialize from the iterator twice"),
        )
    }
}
