use crate::feeds::FeedManager;
use clap::Args;
use rusqlite::Connection;

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(short, long, help = "Show only subscribed feeds")]
    pub subscribed: bool,
}

pub async fn execute(args: ListArgs) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("feeds.db")?;

    let mut feed_manager = FeedManager::new();
    feed_manager.load_from_db(&conn)?;

    if feed_manager.is_empty() {
        println!("No feeds found in the database.");
        return Ok(());
    }

    if args.subscribed {
        let subscribed_feeds = feed_manager.get_subscribed_feeds();
        if subscribed_feeds.is_empty() {
            println!("No subscribed feeds found.");
        } else {
            println!("Subscribed feeds:");
            for feed in subscribed_feeds {
                let title = feed.title.as_ref().unwrap_or(&feed.url);
                println!("  {} * {}", feed.id, title);
            }
        }
    } else {
        let all_feeds = feed_manager.get_all_feeds();
        let total_count = all_feeds.len();

        println!("All feeds:");
        for feed in all_feeds {
            let title = feed.title.as_ref().unwrap_or(&feed.url);
            let marker = if feed.is_subscribed { "*" } else { " " };
            println!(" {} {} {}", feed.id, marker, title);
        }

        let subscribed_count = feed_manager.get_subscribed_feeds().len();
        println!();
        println!(
            "Total: {} feeds ({} subscribed)",
            total_count, subscribed_count
        );
        println!("Note: Subscribed feeds are marked with *");
    }

    Ok(())
}
