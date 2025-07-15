use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Feed {
    pub url: String,
    pub title: Option<String>,
    pub subscribed: bool,
}

impl Feed {
    pub fn new(url: String, title: Option<String>, subscribed: bool) -> Self {
        Self {
            url,
            title,
            subscribed,
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

    pub fn add_feed(&mut self, url: String, title: Option<String>, subscribed: bool) {
        let feed = Feed::new(url.clone(), title, subscribed);
        self.feeds.insert(url, feed);
    }

    pub fn get_all_feeds(&self) -> Vec<&Feed> {
        self.feeds.values().collect()
    }

    pub fn get_subscribed_feeds(&self) -> Vec<&Feed> {
        self.feeds.values().filter(|feed| feed.subscribed).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.feeds.is_empty()
    }

    // For demo purposes, let's add some sample feeds
    pub fn load_sample_data(&mut self) {
        // Comment out to test empty state
        self.add_feed(
            "https://feeds.feedburner.com/oreilly/radar".to_string(),
            Some("O'Reilly Radar".to_string()),
            true,
        );
        self.add_feed(
            "https://rss.cnn.com/rss/edition.rss".to_string(),
            Some("CNN Top Stories".to_string()),
            false,
        );
        self.add_feed(
            "https://feeds.bbci.co.uk/news/rss.xml".to_string(),
            Some("BBC News".to_string()),
            true,
        );
        self.add_feed(
            "https://techcrunch.com/feed/".to_string(),
            Some("TechCrunch".to_string()),
            false,
        );
    }

    // pub fn load_sample_data_no_subscriptions(&mut self) {
    //     // Load feeds but none are subscribed - for testing
    //     self.add_feed(
    //         "https://rss.cnn.com/rss/edition.rss".to_string(),
    //         Some("CNN Top Stories".to_string()),
    //         false,
    //     );
    //     self.add_feed(
    //         "https://techcrunch.com/feed/".to_string(),
    //         Some("TechCrunch".to_string()),
    //         false,
    //     );
    // }
}

impl Default for FeedManager {
    fn default() -> Self {
        Self::new()
    }
}
