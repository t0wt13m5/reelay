use crate::controllers::database::operations::{get_feed_by_id, get_items_by_feed_id};
use clap::Args;
use rusqlite::Connection;

#[derive(Args, Debug)]
pub struct ShowArgs {
    #[arg(value_name = "ID", help = "The ID of the feed to display")]
    pub id: i32,
}

pub async fn execute(args: ShowArgs) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("feeds.db")?;

    match get_feed_by_id(&conn, args.id)? {
        Some(feed) => {
            let title = feed.title.as_ref().unwrap_or(&feed.url);
            println!("{}", title);
            println!("{}", "─".repeat(title.len()));
            println!();

            let items = get_items_by_feed_id(&conn, feed.id)?;

            if items.is_empty() {
                println!("No items found for this feed.");
                println!(
                    "Try running 'reelay fetch {}' to update the feed.",
                    feed.url
                );
            } else {
                for (index, item) in items.iter().enumerate() {
                    println!("{:2}. {}", index + 1, item.title);

                    if let Some(published) = &item.published {
                        print!("{} |", published);
                    }
                    println!("{}", item.link);

                    if let Some(description) = &item.description {
                        let desc = if description.len() > 100 {
                            format!("{}...", &description[..97])
                        } else {
                            description.clone()
                        };
                        println!("{}", desc);
                    }
                    println!();
                }

                println!("Displayed {} entries.", items.len());
            }
        }
        None => {
            println!("No feed found with ID: {}", args.id);
            std::process::exit(1);
        }
    }

    Ok(())
}
