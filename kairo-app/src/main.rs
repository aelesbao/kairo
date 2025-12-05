use clap::Parser;
use console::style;
use kairo_core::{Url, UrlHandlerApp};

mod app;

/// Kiro
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    /// The URL to open.
    url: Url,

    /// Paths to search for desktop entries.
    #[arg(long, default_value = None, global = true)]
    search_paths: Option<Vec<std::path::PathBuf>>,

    #[command(flatten)]
    verbose: clap_verbosity::Verbosity<clap_verbosity::WarnLevel>,
}

impl Cli {
    fn run(&self) -> anyhow::Result<()> {
        pretty_env_logger::formatted_builder()
        // .filter_level(args.verbose.log_level_filter())
        .filter(
            Some(env!("CARGO_CRATE_NAME")),
            self.verbose.log_level_filter(),
        )
        .init();

        let apps =
            UrlHandlerApp::handlers_for_scheme(self.url.scheme(), None, self.search_paths.clone())?;

        app::run(self.url.clone(), apps)?;

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = cli.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn open_with_app(app: &UrlHandlerApp, url: Url) -> kairo_core::Result<u32> {
    println!("Opening URL with {}...", style(&app.name).bold().green());
    app.open_url(url)
}
