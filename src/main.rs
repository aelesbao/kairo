use clap::{Parser, Subcommand};
use clap_verbosity::{Verbosity, WarnLevel};
use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use kiro::{App, Result};

/// Kiro
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
#[command(next_line_help = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    verbose: Verbosity<WarnLevel>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lists all applications that can handle the given URL scheme.
    List {
        /// The URL to query.
        #[arg(
            short,
            long,
            conflicts_with("scheme"),
            required_unless_present("scheme")
        )]
        url: Option<url::Url>,

        /// The URL scheme to query.
        #[arg(short, long, conflicts_with("url"), required_unless_present("url"))]
        scheme: Option<String>,
    },

    /// Opens the given URL with one of its associated applications.
    Open {
        /// The URL to open.
        url: url::Url,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    pretty_env_logger::formatted_builder()
        .filter_level(args.verbose.log_level_filter())
        .init();

    match args.command {
        Commands::List { url, scheme } => {
            let scheme = match (url, scheme) {
                (Some(url), _) => url.scheme().to_string(),
                (_, Some(scheme)) => scheme,
                _ => unreachable!(),
            };

            let apps = App::handlers_for_scheme(&scheme, None, None)?;
            println!(
                "{: <16} {}",
                style("App ID").bold().green(),
                style("Name").bold().green()
            );
            for app in apps {
                println!("{:<16} {}", app.appid, app.name);
            }
        }

        Commands::Open { url } => {
            let apps = App::handlers_for_scheme(url.scheme(), None, None)?;
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
                println!(
                    "Opening URL with {}...",
                    style(&apps[selection].name).bold().green()
                );
                apps[selection].open_url(url)?;
            }
        }
    }

    Ok(())
}
