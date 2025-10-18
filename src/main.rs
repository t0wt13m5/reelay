use clap::{Parser, Subcommand};

mod args;
mod controllers;
mod feeds;
use args::{FetchArgs, ListArgs};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Fetch(FetchArgs),
    List(ListArgs),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    controllers::database::path_handling::check_if_db_path_exists();

    match cli.command {
        Commands::Fetch(args) => {
            args::fetch::execute(args).await?;
        }
        Commands::List(args) => {
            args::list::execute(args).await?;
        }
    }

    Ok(())
}
