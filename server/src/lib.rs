mod response;
mod error;
mod db;
pub mod handler;
mod model;

pub type Result<T> = std::result::Result<T, error::AppError>;

pub use crate::response::Response;
