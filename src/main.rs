use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(
        short,
        long,
        value_name = "URL",
        help = "The URL to fetch the RSS feed from"
    )]
    fetch: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();
    println!("Fetching RSS feed from: {}", args.fetch);

    let client = reqwest::Client::new();

    let response = client.get(&args.fetch).send().await?;

    if response.status().is_success() {
        let content = response.text().await?;
        println!("RSS Feed Content:");
        println!("{}", content);
    } else {
        eprintln!("Failed to fetch RSS feed. Status: {}", response.status());
        std::process::exit(1);
    }

    Ok(())
}
