use axum::{http::StatusCode, response::IntoResponse};
use tower_http;

#[derive(Debug)]
pub enum Error {
    LoginFail,
}

pub type Result<T> = core::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> Error: {:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong.").into_response()
    }
}
