use crate::controllers::database::models::Feed;
use crate::controllers::database::operations::{initiate_db, save_feed};
use clap::Args;
use feed_rs::parser;
use reqwest::Client;
use rusqlite::Connection;
#[derive(Args, Debug)]
pub struct FetchArgs {
    #[arg(value_name = "URL", help = "The URL to fetch the RSS feed from")]
    pub url: String,
}

pub async fn execute(args: FetchArgs) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(&args.url).send().await?;

    if !response.status().is_success() {
        eprintln!("Failed to fetch RSS feed. Status: {}", response.status());
        std::process::exit(1);
    }

    let content = response.bytes().await?;
    let feed_data = parser::parse(&content[..])?;

    let title = feed_data
        .title
        .clone()
        .map(|t| t.content)
        .or_else(|| Some(args.url.clone()));

    let conn = Connection::open("feeds.db")?;
    initiate_db(&conn)?;

    let feed = Feed {
        id: 0,
        url: args.url.clone(),
        title: title.clone(),
        last_updated: None,
        is_subscribed: true,
    };

    save_feed(&conn, &feed)?;
    println!("Feed saved.");
    println!();

    println!("{}", title.as_deref().unwrap_or("Untitled Feed"));
    println!(
        "{}",
        "─".repeat(title.as_deref().map(|t| t.len()).unwrap_or(10))
    );
    println!();

    let entries_to_show = 10.min(feed_data.entries.len());
    for (i, entry) in feed_data.entries.iter().take(entries_to_show).enumerate() {
        let entry_title = entry
            .title
            .as_ref()
            .map(|t| t.content.clone())
            .unwrap_or_else(|| "(no title)".to_string());

        let link = entry
            .links
            .first()
            .map(|l| l.href.clone())
            .unwrap_or_else(|| "(no link)".to_string());

        let published = entry
            .published
            .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "unknown date".to_string());

        println!("{:>2}. {}", i + 1, entry_title);
        println!("{} |{}", published, link);

        if let Some(summary) = &entry.summary {
            let text = summary.content.trim();
            if !text.is_empty() {
                println!("{}", truncate(text, 120));
            }
        }

        println!();
    }

    println!("Displayed {entries_to_show} entries.\n");

    Ok(())
}

fn truncate(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[..max_len])
    } else {
        text.to_string()
    }
}
