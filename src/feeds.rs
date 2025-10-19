use crate::controllers::database::models::*;
use crate::controllers::database::operations::read_all_feeds;
use rusqlite::Connection;
use std::collections::HashMap;

impl Feed {
    #[allow(dead_code)]
    pub fn new(url: String, title: Option<String>, is_subscribed: bool) -> Self {
        Self {
            id: 0,
            url,
            title,
            last_updated: None,
            is_subscribed,
        }
    }
}

pub struct FeedManager {
    feeds: HashMap<String, Feed>,
}

impl FeedManager {
    pub fn new() -> Self {
        Self {
            feeds: HashMap::new(),
        }
    }

    pub fn load_from_db(&mut self, conn: &Connection) -> rusqlite::Result<()> {
        let feeds = read_all_feeds(conn)?;
        self.feeds = feeds
            .into_iter()
            .map(|feed| (feed.url.clone(), feed))
            .collect();
        Ok(())
    }

    pub fn get_all_feeds(&self) -> Vec<&Feed> {
        self.feeds.values().collect()
    }

    pub fn get_subscribed_feeds(&self) -> Vec<&Feed> {
        self.feeds
            .values()
            .filter(|feed| feed.is_subscribed)
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.feeds.is_empty()
    }
}

impl Default for FeedManager {
    fn default() -> Self {
        Self::new()
    }
}
