#![allow(unused_imports)]

use std::io::Read;
use std::sync::Arc;

use anyhow::Context;
use clap::{ArgAction, Parser, ValueEnum};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use shared::display::DisplayPreference;
use tracing::level_filters::LevelFilter;

#[path = "../shared/mod.rs"]
mod shared;

use shared::format::{Format, ParseSettings};
use shared::schema::Schema;

/// Robustly parse incomplete input (e.g. JSON), and reserialize in some format
#[derive(Parser, Debug, Clone)]
#[command(version)]
struct Args {
    /// The data format to parse.
    #[clap(short, long, default_value_t)]
    #[arg(value_enum)]
    format: Format,

    /// Parse according to this format. Default: any JSON.
    #[clap(long, default_value_t)]
    #[arg(value_enum)]
    schema: Schema,

    /// How to display the parsed data.
    #[clap(short, long, default_value = "json-raw")]
    output: Vec<DisplayPreference>,

    /// Whether to add a randomized trailer to the input before parsing.
    ///
    /// For JSON, this enables incremental strings. For YAML, this
    /// does too, and is really important to prevent flickering.
    ///
    /// Randomized trailers are format-specific.
    #[clap(long, default_value_t = true, action = ArgAction::Set)]
    use_random_trailer: bool,
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
        .init();

    let args = Args::parse();

    let mut display = DisplayPreference::init(&args.output).await;

    let mut input = vec![];
    std::io::stdin()
        .read_to_end(&mut input)
        .context("could not read input")?;

    let result = args.schema.parse(
        &args.format,
        &ParseSettings {
            use_random_trailer: args.use_random_trailer,
        },
        &input,
    );

    let displayed = display.display(Arc::new(result)).await;
    println!("{displayed}");

    Ok(())
}
