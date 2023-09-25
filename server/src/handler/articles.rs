use std::{collections::HashMap, sync::Arc};

use crate::{
    db::articles::{search_article_by_id, search_articles_archive},
    model::Article,
    AppState, Response,
};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::NaiveDateTime;
use pulldown_cmark::Options;

use super::HandlerResult;

pub async fn handle_search_article_by_id(
    Path(article_id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> HandlerResult<Article> {
    let mut article = search_article_by_id(article_id, &state.pool).await?;
    let markdown_input = article.article_content;
    let options = Options::all();

    let parser = pulldown_cmark::Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser.into_iter());

    article.article_content = html_output;

    Ok(Json(Response::ok(article)))
}

pub async fn handle_search_articles_archive(
    State(state): State<Arc<AppState>>,
) -> HandlerResult<HashMap<NaiveDateTime, Vec<Article>>> {
    let res = search_articles_archive(&state.pool).await?;

    Ok(Json(Response::ok(res)))
}
