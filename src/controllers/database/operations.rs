use super::models::*;
use rusqlite::{params, Connection, Result};

pub fn initiate_db(connection: &Connection) -> Result<()> {
    let sql = Sql::new();
    let feeds_schema = sql.get_sql(SqlType::Feed);
    let items_schema = sql.get_sql(SqlType::Item);
    // TODO: @t0wt13m5 make params modular
    let params = [];

    connection.execute(feeds_schema, params)?;
    connection.execute(items_schema, params)?;

    Ok(())
}

pub fn read_all_feeds(conn: &Connection) -> Result<Vec<Feed>> {
    let mut stmt = conn.prepare("SELECT id, url, title, lastUpdated, isSubscribed FROM feeds")?;

    let feed_iter = stmt.query_map([], |row| {
        Ok(Feed {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            last_updated: row.get(3)?,
            is_subscribed: row.get(4)?,
        })
    })?;

    let mut feeds = Vec::new();
    for feed in feed_iter {
        feeds.push(feed?);
    }

    Ok(feeds)
}

pub fn save_feed(conn: &Connection, feed: &Feed) -> Result<i64> {
    conn.execute(
        "INSERT INTO feeds (url, title, lastUpdated, isSubscribed)
         VALUES (?1, ?2, datetime('now'), ?3)
         ON CONFLICT(url) DO UPDATE SET
             title = excluded.title,
             isSubscribed = excluded.isSubscribed,
             lastUpdated = datetime('now')",
        params![feed.url, feed.title, feed.is_subscribed],
    )?;

    Ok(conn.last_insert_rowid())
}
