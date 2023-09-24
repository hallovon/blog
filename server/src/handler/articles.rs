use axum::{extract::Path, Json};
use pulldown_cmark::Options;
use sqlx::postgres::PgPoolOptions;

use crate::{db::articles::search_article_by_id, model::Article, Response};

use super::HandlerResult;

pub async fn handle_search_article_by_id(Path(article_id): Path<i32>) -> HandlerResult<Article> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@127.0.0.1:5432/blog")
        .await
        .unwrap();

    let mut article = search_article_by_id(article_id, pool).await?;

    let markdown_input = article.article_content;

    let options = Options::all();

    let parser = pulldown_cmark::Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser.into_iter());

    article.article_content = html_output;

    Ok(Json(Response::ok(article)))
}
