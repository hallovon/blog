use axum::Json;

use crate::Response;

pub mod test;

type HandlerResult<T> = crate::Result<Json<Response<T>>>;


