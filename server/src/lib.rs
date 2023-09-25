use sqlx::PgPool;

mod db;
mod error;
pub mod handler;
mod model;
mod response;

pub type Result<T> = std::result::Result<T, error::AppError>;
pub use crate::response::Response;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
