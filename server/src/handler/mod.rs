use axum::Json;

use crate::Response;

pub mod articles;

type HandlerResult<T> = crate::Result<Json<Response<T>>>;


