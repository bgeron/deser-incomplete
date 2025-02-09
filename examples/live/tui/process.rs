use std::sync::Arc;

use anyhow::Context;
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget as _, Wrap};
use ratatui::TerminalOptions;
use tokio::io::AsyncReadExt;
use tokio::sync::{mpsc, watch};
use tokio_stream::wrappers::WatchStream;
use tokio_stream::StreamExt as _;
use tokio_util::bytes;
use tokio_util::io::ReaderStream;
use tracing::error;
use tui_textarea::TextArea;

use crate::shared::display::{ActiveDisplay, DisplayPreference};
use crate::shared::format::ParseSettings;
use crate::Args;

pub(crate) fn compute_displays_from_inputs(
    args: Args,
    inputs: watch::Receiver<Option<String>>,
    mut display: Box<dyn ActiveDisplay>,
) -> watch::Receiver<Text<'static>> {
    let (tx, rx) = watch::channel("initializing...".into());

    tokio::spawn(async move {
        let mut inputs_rx = WatchStream::new(inputs);
        while let Some(input) = inputs_rx.next().await {
            let display = compute(&args, input, &mut display).await;

            tx.send(display).unwrap();
        }
    });

    rx
}

async fn compute(
    args: &Args,
    input: Option<String>,
    display: &mut Box<dyn ActiveDisplay>,
) -> Text<'static> {
    match input {
        None => Text::styled(
            "No input yet. Start typing to get a deserialization.\n\n",
            Style::default().gray().bold().italic(),
        ),
        Some(input) => {
            let result = args.schema.parse(
                &args.format,
                &ParseSettings {
                    use_random_trailer: args.use_random_trailer,
                },
                input.as_bytes(),
            );
            let displayed = display.display(Arc::new(result)).await;
            ansi_to_tui::IntoText::into_text(&displayed)
                .unwrap_or_else(|_| "could not interpret terminal output".into())
        }
    }
}
