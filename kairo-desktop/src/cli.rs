use clap::Parser;
use kairo_core::{Url, UrlHandlerApp};

use crate::app;

/// Kairo
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    /// The URL to open.
    url: Url,

    /// Paths to search for desktop entries.
    #[arg(long, default_value = None, global = true)]
    search_paths: Option<Vec<std::path::PathBuf>>,

    #[command(flatten)]
    verbose: clap_verbosity::Verbosity<clap_verbosity::WarnLevel>,

    /// Enable UI debug mode to explain the elements layout.
    #[cfg(debug_assertions)]
    #[arg(long, default_value_t = false, global = true)]
    pub debug_ui: bool,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn run(&self) -> anyhow::Result<()> {
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Info)
            .filter(
                Some(env!("CARGO_CRATE_NAME")),
                self.verbose.log_level_filter(),
            )
            .init();

        let apps =
            UrlHandlerApp::handlers_for_scheme(self.url.scheme(), None, self.search_paths.clone())?;

        app::run(self.url.clone(), apps, self.debug_ui)?;

        Ok(())
    }
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::new();
    cli.run()
}
