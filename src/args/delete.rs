use crate::controllers::database::operations::{delete_feed_with_items, get_feed_by_id};
use clap::Args;
use rusqlite::Connection;

#[derive(Args, Debug)]
pub struct DeleteArgs {
    #[arg(value_name = "ID", help = "The ID of the feed to delete")]
    pub id: i32,

    #[arg(short, long, help = "Skip confirmation prompt")]
    pub force: bool,
}

pub async fn execute(args: DeleteArgs) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("feeds.db")?;
    let feed = match get_feed_by_id(&conn, args.id)? {
        Some(feed) => feed,
        None => {
            println!("No feed found with ID: {}", args.id);
            std::process::exit(1);
        }
    };

    let feed_title = feed.title.as_ref().unwrap_or(&feed.url);

    if !args.force {
        println!("Are you sure you want to delete the following feed?");
        println!("  ID: {}", feed.id);
        println!("  Title: {}", feed_title);
        println!("  URL: {}", feed.url);
        println!();
        println!("This will permanently delete the feed and all its items.");
        print!("Continue? [y/N]: ");

        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("Deletion cancelled.");
            return Ok(());
        }
    }
    match delete_feed_with_items(&conn, args.id) {
        Ok((feeds_deleted, items_deleted)) => {
            if feeds_deleted > 0 {
                println!("Successfully deleted feed: {}", feed_title);
                if items_deleted > 0 {
                    println!("Also deleted {} associated items.", items_deleted);
                }
            } else {
                println!("Feed not found or already deleted.");
            }
        }
        Err(e) => {
            eprintln!("Error deleting feed: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
