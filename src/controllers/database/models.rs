#[derive(Debug)]
pub struct Feed {
    pub id: i32,
    pub url: String,
    pub title: Option<String>,
    pub last_updated: Option<String>,
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
