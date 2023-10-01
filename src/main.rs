use std::net::SocketAddr;

use axum::{self, Router};
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing;
use serde::Deserialize;
use tokio;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let routes = axum::Router::new().merge(routes_hello());
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

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", routing::get(handler_hello))
        .route("/hello2/:name", routing::get(handler_hello2))
}