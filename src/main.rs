use std::net::SocketAddr;

use axum::extract::{Path, Query};
use axum::http::response;
use axum::response::{Html, IntoResponse, Response};
use axum::{routing, middleware};
use axum::{self, Router};
use serde::Deserialize;
use tokio;
use tower_http::services::ServeDir;

mod error;
pub use error::{Result, Error};
mod web;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let routes = axum::Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static())
        .layer(middleware::map_response(main_response_mapper));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8095));
    println!("->> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("Starting a server should work.");
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    Html(format!(
        "<p>Hello, {who}!</p>",
        who = params.name.as_deref().unwrap_or("World!")
    ))
}

async fn handler_hello2(Path(user_name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2", "HANDLER");
    Html(format!("<p>Hello, {user_name}!</p>"))
}

async fn main_response_mapper(response_in: Response) -> Response {
    println!("");
    response_in
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", routing::get(handler_hello))
        .route("/hello2/:name", routing::get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", ServeDir::new("./"))
}