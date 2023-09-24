use std::net::SocketAddr;

use axum::{http::Method, routing::get, Router};

use server::handler::articles::handle_search_article_by_id;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root))
        .route("/api/blog/:article_id", get(handle_search_article_by_id));

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
