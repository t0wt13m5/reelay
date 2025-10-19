#[derive(Debug)]
pub struct Feed {
    pub id: i32,
    pub url: String,
    pub title: Option<String>,
    pub last_updated: Option<String>,
    pub is_subscribed: bool,
}

#[derive(Debug)]
pub struct Item {
    pub id: i32,
    pub feed_id: i32,
    pub title: String,
    pub link: String,
    pub published: Option<String>,
    pub description: Option<String>,
}

pub enum SqlType {
    Feed,
    Item,
}

pub struct Sql {
    pub feed_schema: &'static str,
    pub item_schema: &'static str,
}
