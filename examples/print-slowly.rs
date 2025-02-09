use std::io::Write as _;
use std::time::Duration;

use clap::Parser;
use futures::FutureExt;
use tokio::io::{stdin, AsyncReadExt, BufReader};
use unicode_segmentation::UnicodeSegmentation as _;
use util::{pop_parsed_from_front, MAXIMUM_SIZE_OF_CODEPOINT};

#[path = "live/util.rs"]
mod util;

const MAX_BYTES_IN_GRAPHEME_CLUSTER: usize = 128;

/// Print the input, but slowly.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about=None)]
struct Args {
    /// Duration between output characters in seconds.
    #[clap(short('t'), long, default_value_t = 0.05)]
    delay: f32,

    #[clap(long, default_value_t = 1.5)]
    delay_at_start: f32,

    #[clap(long, default_value_t = 1.5)]
    delay_at_exit: f32,
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
        .init();

    let args = Args::parse();
    let delay = Duration::from_secs_f32(args.delay);

    let mut buf_str: String = String::new();
    let mut buf_byte: Vec<u8> = Vec::with_capacity(128);

    let mut stdin = BufReader::new(stdin());

    tokio::spawn(async {
        tokio::signal::ctrl_c().await.unwrap();
        std::process::exit(0);
    });

    tokio::time::sleep(Duration::from_secs_f32(args.delay_at_start)).await;

    loop {
        // Best effort attempt to read bytes when available.
        if buf_byte.len() < buf_byte.capacity() {
            if let Some(happened) = stdin.read_buf(&mut buf_byte).now_or_never() {
                happened.unwrap();
            }
        }

        // Move bytes to string where possible
        loop {
            if buf_str.len() >= MAX_BYTES_IN_GRAPHEME_CLUSTER {
                break;
            }
            match pop_parsed_from_front(&mut buf_byte, MAXIMUM_SIZE_OF_CODEPOINT, |bytes| {
                std::str::from_utf8(bytes).map(ToOwned::to_owned).ok()
            }) {
                Some(char) => {
                    buf_str.push_str(&char);
                }
                _ => break,
            }
        }

        // Now we should ideally have a large grapheme cluster at the start of buf_str.
        match buf_str.graphemes(true).next() {
            None => {
                // Force make progress on getting input
                let n_bytes = stdin.read_buf(&mut buf_byte).await.unwrap();
                if n_bytes == 0 {
                    // Input seems closed
                    tokio::time::sleep(Duration::from_secs_f32(args.delay_at_exit)).await;
                    return;
                }
            }
            Some(grapheme) => {
                let size = grapheme.len();
                print!("{grapheme}");
                std::io::stdout().flush().unwrap();
                buf_str.drain(0..size);

                tokio::time::sleep(delay).await;
            }
        }
    }
}
