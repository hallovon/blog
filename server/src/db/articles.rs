use std::collections::HashMap;

use crate::model::{Article, DateTime};
use crate::Result;
use chrono::NaiveDateTime;
use sqlx::PgPool;

pub async fn search_article_by_id(article_id: i32, pool: &PgPool) -> Result<Article> {
    let res = sqlx::query_as("select * from articles where article_id = $1")
        .bind(article_id)
        .fetch_one(pool)
        .await
        .unwrap();

    Ok(res)
}

pub async fn search_article_month_list(pool: &PgPool) -> Result<Vec<DateTime>> {
    let res = sqlx::query_as(
        r#"
        SELECT DISTINCT
            date_trunc( 'month', modify_time ) AS datetime 
        FROM
            articles 
        ORDER BY
            date_trunc( 'month', modify_time ) DESC;"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(res)
}

pub async fn search_articles_archive(pool: &PgPool) -> Result<HashMap<NaiveDateTime, Vec<Article>>> {
    let months = search_article_month_list(pool).await?;

    let mut res = HashMap::new();

    for DateTime(datetime) in months {
        let articles = sqlx::query_as(
            r#"
            SELECT
                * 
            FROM
                articles 
            WHERE
                date_trunc( 'month', modify_time ) = $1 
            ORDER BY
                modify_time;"#,
        )
        .bind(datetime)
        .fetch_all(pool)
        .await?;

        res.insert(datetime, articles);
    }

    Ok(res)
}
