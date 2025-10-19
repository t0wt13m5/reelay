use super::models::*;
use rusqlite::{params, Connection, Result};

pub fn initiate_db(connection: &Connection) -> Result<()> {
    let sql = Sql::new();
    let feeds_schema = sql.get_sql(SqlType::Feed);
    let items_schema = sql.get_sql(SqlType::Item);
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

pub fn get_feed_by_id(conn: &Connection, feed_id: i32) -> Result<Option<Feed>> {
    let mut stmt =
        conn.prepare("SELECT id, url, title, lastUpdated, isSubscribed FROM feeds WHERE id = ?1")?;

    let mut feed_iter = stmt.query_map([feed_id], |row| {
        Ok(Feed {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            last_updated: row.get(3)?,
            is_subscribed: row.get(4)?,
        })
    })?;

    match feed_iter.next() {
        Some(feed) => Ok(Some(feed?)),
        None => Ok(None),
    }
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

pub fn get_items_by_feed_id(conn: &Connection, feed_id: i32) -> Result<Vec<Item>> {
    let mut stmt = conn.prepare("SELECT id, feedId, title, link, publishedAt, description FROM entries WHERE feedId = ?1 ORDER BY publishedAt DESC")?;

    let item_iter = stmt.query_map([feed_id], |row| {
        Ok(Item {
            id: row.get(0)?,
            feed_id: row.get(1)?,
            title: row.get(2)?,
            link: row.get(3)?,
            published: row.get(4)?,
            description: row.get(5)?,
        })
    })?;

    let mut items = Vec::new();
    for item in item_iter {
        items.push(item?);
    }

    Ok(items)
}

pub fn save_item(conn: &Connection, item: &Item) -> Result<i64> {
    conn.execute(
        "INSERT INTO entries (feedId, title, link, publishedAt, description)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(link) DO UPDATE SET
             title = excluded.title,
             publishedAt = excluded.publishedAt,
             description = excluded.description",
        params![
            item.feed_id,
            item.title,
            item.link,
            item.published,
            item.description
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_feed_by_url(conn: &Connection, url: &str) -> Result<Option<Feed>> {
    let mut stmt =
        conn.prepare("SELECT id, url, title, lastUpdated, isSubscribed FROM feeds WHERE url = ?1")?;

    let mut feed_iter = stmt.query_map([url], |row| {
        Ok(Feed {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            last_updated: row.get(3)?,
            is_subscribed: row.get(4)?,
        })
    })?;

    match feed_iter.next() {
        Some(feed) => Ok(Some(feed?)),
        None => Ok(None),
    }
}

pub fn delete_feed_items(conn: &Connection, feed_id: i32) -> Result<usize> {
    let deleted_count = conn.execute("DELETE FROM entries WHERE feedId = ?1", params![feed_id])?;
    Ok(deleted_count)
}

pub fn delete_feed(conn: &Connection, feed_id: i32) -> Result<usize> {
    let deleted_count = conn.execute("DELETE FROM feeds WHERE id = ?1", params![feed_id])?;
    Ok(deleted_count)
}

pub fn delete_feed_with_items(conn: &Connection, feed_id: i32) -> Result<(usize, usize)> {
    let items_deleted = delete_feed_items(conn, feed_id)?;
    let feeds_deleted = delete_feed(conn, feed_id)?;

    Ok((feeds_deleted, items_deleted))
}
