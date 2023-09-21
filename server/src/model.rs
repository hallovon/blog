use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Article {
    article_id: i32,
    title: String,
    create_time: NaiveDateTime,
    modify_time: NaiveDateTime,
    topic: String,
    pub article_content: String,
}


#[derive(Serialize, FromRow)]
pub struct Topic {
    topic_id: i32,
    topic_name: String,
}
