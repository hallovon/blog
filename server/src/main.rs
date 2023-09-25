use std::{env, net::SocketAddr, sync::Arc};

use axum::{http::Method, routing::get, Router};

use server::{
    handler::articles::{handle_search_article_by_id, handle_search_articles_archive},
    AppState,
};

use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("no configure in file .env");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("database connection failed!");

    let shared_state = Arc::new(AppState { pool });

    let router = Router::new()
        .route("/api", get(root))
        .route("/api/article/:article_id", get(handle_search_article_by_id))
        .route("/api/article/archive", get(handle_search_articles_archive))
        .with_state(shared_state);

    let app = Router::new().merge(router).layer(
        CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_origin(Any),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "hello world"
}
