use crate::model::Article;
use crate::Result;
use sqlx::PgPool;

pub async fn search_article_by_id(article_id: i32, pool: PgPool) -> Result<Article> {
    let res = sqlx::query_as("select * from articles where article_id = $1")
        .bind(article_id)
        .fetch_one(&pool)
        .await.unwrap();

    Ok(res)
}
