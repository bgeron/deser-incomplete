use std::fmt::Write as _;
use std::sync::Arc;

use clap::ValueEnum;
use futures::future::{join_all, BoxFuture};
use futures::FutureExt as _;
use nu::Prefs;

use super::format::{ParseOk, ParseResult};

mod debug;
mod json;
mod nu;
#[cfg(feature = "serde_yaml")]
mod yaml;

/// How to display the parsed value.
///
/// By default, this will use nushell, falling back to Debug.
#[derive(Debug, Clone, ValueEnum)]

pub enum DisplayPreference {
    NushellLight,
    NushellDark,
    Debug,
    Json,
    JsonRaw,
    Yaml,
}

impl DisplayPreference {
    /// Display types need to be initialized to do some startup.
    pub async fn init(methods: &[Self]) -> Box<dyn ActiveDisplay> {
        if methods.is_empty() {
            // Default.

            // Autodetect nu. Otherwise fall back to debug.
            match nu::Display::new_if_nu_installed(nu::Prefs::light()).await {
                Some(nu) => Box::new(vec![
                    Box::new(nu) as Box<dyn ActiveDisplay>,
                    Box::new(debug::Display { prefix: "" }),
                ]),
                None => Box::new(debug::Display {
                    prefix: "Nushell does not seem to be present, falling back to Debug.\n\n",
                }),
            }
        } else {
            Box::new(
                methods
                    .iter()
                    .map(|method| match method {
                        DisplayPreference::NushellLight => {
                            let prefs=Prefs::light();
                            Box::new(nu::Display::new_always(prefs)) as _
                    },
                        DisplayPreference::NushellDark => {
                            let prefs=Prefs::dark();
                            Box::new(nu::Display::new_always(prefs)) as _
                    },
                        DisplayPreference::Debug => Box::new(debug::Display { prefix: "" }) as _,
                        DisplayPreference::Json => Box::new(json::Display { pretty: true }) as _,
                        DisplayPreference::JsonRaw => {
                            Box::new(json::Display { pretty: false }) as _
                        }
                        #[cfg(feature = "serde_yaml")]
                        DisplayPreference::Yaml => Box::new(yaml::Display) as _,
                        #[cfg(not(feature = "serde_yaml"))]
                        DisplayPreference::Yaml => {
                            panic!(
                                "Please enable --features serde_yaml to format with YAML, or run cargo with --all-features)"
                            )
                        }
                    })
                    .collect::<Vec<Box<dyn ActiveDisplay>>>(),
            )
        }
    }
}

pub trait ActiveDisplay: Send {
    fn descriptor(&self) -> Option<&str>;
    fn display_ok(&mut self, value: ParseOk) -> BoxFuture<'_, String>;
    fn display(&mut self, value: Arc<ParseResult>) -> BoxFuture<'_, String> {
        async move {
            match &*value {
                Ok(ok) => self.display_ok((*ok).clone()).await,
                Err(err) => format!("Error: {err}"),
            }
        }
        .boxed()
    }
}

impl<D> ActiveDisplay for Box<D>
where
    D: ActiveDisplay + ?Sized,
{
    fn descriptor(&self) -> Option<&str> {
        (**self).descriptor()
    }

    fn display_ok(&mut self, value: ParseOk) -> BoxFuture<'_, String> {
        (**self).display_ok(value)
    }
    fn display(&mut self, value: Arc<ParseResult>) -> BoxFuture<'_, String> {
        (**self).display(value)
    }
}

impl<D> ActiveDisplay for Vec<D>
where
    D: ActiveDisplay,
{
    fn descriptor(&self) -> Option<&str> {
        panic!("you're not supposed to nest vectors of vectors of ActiveDisplay")
    }

    fn display_ok(&mut self, value: ParseOk) -> BoxFuture<'_, String> {
        async move {
            let outputs = join_all(
                self.iter_mut()
                    .map(|d| async {
                        let displayed = d.display_ok(value.clone()).await;
                        (d.descriptor(), displayed)
                    })
                    .collect::<Vec<_>>(),
            )
            .await;

            let mut buf = String::new();
            for (i, (desc, displayed)) in outputs.into_iter().enumerate() {
                if i > 0 {
                    // Write into buffer always succeeds.
                    write!(&mut buf, "\n\n").unwrap();
                    if let Some(desc) = desc {
                        writeln!(&mut buf, "{desc}:").unwrap();
                    }
                }
                write!(&mut buf, "{displayed}").unwrap();
            }
            buf
        }
        .boxed()
    }
}
