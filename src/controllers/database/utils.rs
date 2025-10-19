use super::models::*;
use super::schema::*;

impl Sql {
    pub fn new() -> Self {
        Self {
            feed_schema: FEED_SCHEMA,
            item_schema: ITEM_SCHEMA,
        }
    }

    pub fn get_sql(&self, sql_type: SqlType) -> &'static str {
        match sql_type {
            SqlType::Feed => self.feed_schema,
            SqlType::Item => self.item_schema,
        }
    }
}
