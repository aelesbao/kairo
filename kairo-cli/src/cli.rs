use std::path::PathBuf;

use clap::{Parser, Subcommand};
use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use kairo_core::{Url, UrlHandlerApp};

/// Kairo
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Paths to search for desktop entries.
    #[arg(long, default_value = None, global = true)]
    search_paths: Option<Vec<std::path::PathBuf>>,

    #[command(flatten)]
    verbose: clap_verbosity::Verbosity<clap_verbosity::WarnLevel>,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn run(&self) -> anyhow::Result<()> {
        pretty_env_logger::formatted_builder()
            .filter_level(self.verbose.log_level_filter())
            .init();

        self.command.process()?;

        Ok(())
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lists all applications that can handle a given URL scheme.
    List {
        /// The URL to query (conflicts with --scheme).
        #[arg(
            short,
            long,
            conflicts_with("scheme"),
            required_unless_present("scheme")
        )]
        url: Option<Url>,

        /// The URL scheme to query (conflicts with --url).
        #[arg(short, long, conflicts_with("url"), required_unless_present("url"))]
        scheme: Option<String>,
    },

    /// Opens the given URL with one of its associated applications.
    Open {
        /// The URL to open.
        url: Url,

        /// Opens the URL using the default or last application used without prompting.
        #[arg(long, default_value = "false")]
        no_prompt: bool,
    },
}

impl Commands {
    fn process(&self) -> kairo_core::Result<()> {
        match self {
            Commands::List { url, scheme } => Self::list(url.clone(), scheme.clone(), None),
            Commands::Open { url, no_prompt } => Self::open(url.clone(), None, *no_prompt),
        }
    }

    fn list(
        url: Option<Url>,
        scheme: Option<String>,
        search_paths: Option<Vec<PathBuf>>,
    ) -> kairo_core::Result<()> {
        let scheme = match (url, scheme) {
            (Some(url), _) => url.scheme().to_string(),
            (_, Some(scheme)) => scheme,
            _ => unreachable!(),
        };
        let apps = UrlHandlerApp::handlers_for_scheme(&scheme, None, search_paths)?;

        println!(
            "{: <16} {}",
            style("App ID").bold().green(),
            style("Name").bold().green()
        );

        for app in apps {
            println!("{:<16} {}", app.appid, app.name);
        }

        Ok(())
    }

    fn open(
        url: Url,
        search_paths: Option<Vec<PathBuf>>,
        no_prompt: bool,
    ) -> kairo_core::Result<()> {
        let apps = UrlHandlerApp::handlers_for_scheme(url.scheme(), None, search_paths)?;

        if no_prompt || apps.len() == 1 {
            Self::open_with_app(&apps[0], url)?;
            return Ok(());
        }

        let app_names: Vec<String> = apps
            .iter()
            .map(|app| format!("{:<16} {}", app.appid, app.name))
            .collect();
        let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an application to open the URL with")
        .report(false)
        // TODO: save the last used app as default
        .default(0)
        .items(&app_names)
        .interact_opt()
        .unwrap();

        if let Some(selection) = selection {
            Self::open_with_app(&apps[selection], url)?;
        }

        Ok(())
    }

    fn open_with_app(app: &UrlHandlerApp, url: Url) -> kairo_core::Result<u32> {
        println!("Opening URL with {}...", style(&app.name).bold().green());
        app.open_url(url)
    }
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::new();
    cli.run()
}
