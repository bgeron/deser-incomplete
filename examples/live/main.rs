#![allow(unused_imports)]

use std::io::Read;

use clap::{ArgAction, Parser, ValueEnum};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use shared::display::DisplayPreference;
use tracing::level_filters::LevelFilter;

#[path = "../shared/mod.rs"]
mod shared;
mod tui;
mod util;

use shared::format::Format;
use shared::schema::Schema;

/// Parse input JSON incrementally as it comes in, and show the results
/// live in the terminal.
#[derive(Parser, Debug, Clone)]
#[command(version)]
struct Args {
    #[clap(short, long, default_value_t)]
    #[arg(value_enum)]
    format: Format,

    #[clap(long, default_value_t)]
    #[arg(value_enum)]
    schema: Schema,

    /// How to display the parsed data. Default: use nushell when installed.
    #[clap(short, long)]
    output: Vec<DisplayPreference>,

    /// Whether to add a randomized trailer to the input before parsing.
    ///
    /// For JSON, this enables incremental strings. For YAML, this
    /// does too, and is really important to prevent flickering.
    ///
    /// Randomized trailers are format-specific.
    #[clap(long, default_value_t = true, action = ArgAction::Set)]
    use_random_trailer: bool,

    /// Height of the text user interface. Zero means fullscreen.
    #[clap(long, default_value_t = 24)]
    tui_height: u16,

    /// Wrap text automatically. This is very heuristic, will be wrong on some inputs,
    /// and will mess with the editor state.
    #[clap(long)]
    wrap: bool,
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

    tui::main(args).await
}
