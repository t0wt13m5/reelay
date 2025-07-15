use clap::Args;

#[derive(Args, Debug)]
pub struct FetchArgs {
    #[arg(value_name = "URL", help = "The URL to fetch the RSS feed from")]
    pub url: String,
}

pub async fn execute(args: FetchArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching RSS feed from: {}", args.url);

    let client = reqwest::Client::new();
    let response = client.get(&args.url).send().await?;

    if response.status().is_success() {
        let content = response.text().await?;
        println!("RSS Feed Content:");
        // TODO: dont print the feed, do something with it later on
        println!("{}", content);
    } else {
        eprintln!("Failed to fetch RSS feed. Status: {}", response.status());
        std::process::exit(1);
    }

    Ok(())
}
