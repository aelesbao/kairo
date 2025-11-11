use clap::{Parser, Subcommand};
use kiro::{KiroResult, handlers_for_scheme};

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
            for app in apps {
                println!("{}", app.name);
            }
        }
    }

    Ok(())
}
