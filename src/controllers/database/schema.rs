pub const FEED_SCHEMA: &str = r#"
        CREATE TABLE IF NOT EXISTS feeds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT NOT NULL UNIQUE,
            title TEXT,
            lastUpdated TEXT,
            isSubscribed BOOLEAN NOT NULL
    )
    "#;

pub const ITEM_SCHEMA: &str = r#"
        CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            feedId INTEGER NOT NULL,
            title TEXT NOT NULL,
            link TEXT NOT NULL UNIQUE,
            publishedAt TEXT,
            description TEXT,
            FOREIGN KEY(feedId) REFERENCES feeds(id)
    )
    "#;
