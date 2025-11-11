use clap::{Parser, Subcommand};
use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use kiro::{KiroResult, handlers_for_scheme, open_with_app};

/// Kiro
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
#[command(next_line_help = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
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

fn main() -> KiroResult<()> {
    let args = Args::parse();

    match args.command {
        Commands::List { url, scheme } => {
            let scheme = match (url, scheme) {
                (Some(url), _) => url.scheme().to_string(),
                (_, Some(scheme)) => scheme,
                _ => unreachable!(),
            };

            let apps = handlers_for_scheme(&scheme)?;
            println!(
                "{: <16} {}",
                style("App ID").bold().green(),
                style("Name").bold().green()
            );
            for app in apps {
                println!("{: <16} {}", app.appid, app.name);
            }
        }

        Commands::Open { url } => {
            let apps = handlers_for_scheme(url.scheme())?;

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select an application to open the URL with")
                // TODO: save the last used app as default
                .default(0)
                .items(&apps[..])
                .interact()
                .unwrap();

            let app = apps[selection].clone();

            println!("Opening on {}", style(&app).cyan());
            open_with_app(app, url)?;
        }
    }

    Ok(())
}
