use clap::{Parser, Subcommand};

mod args;
mod feeds;
use args::{FetchArgs, ListArgs};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Fetch(FetchArgs),
    List(ListArgs),
    // Future commands can be added here
    // Subscribe,
    // Unsubscribe,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch(args) => {
            args::fetch::execute(args).await?;
        }
        Commands::List(args) => {
            args::list::execute(args).await?;
        } // Future command handlers can be added here
          // Commands::Subscribe(args) => {
          //     args::subscribe::execute(args).await?;
          // }
    }

    Ok(())
}
