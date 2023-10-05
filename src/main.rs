use std::net::SocketAddr;

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::{self, Router};
use axum::{middleware, routing};
use serde::Deserialize;
use tokio;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
pub use error::{Error, Result};

use crate::model::ModelController;
mod model;
mod web;
mod ctx;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let mc = ModelController::new().await.unwrap();
    let tickets_api_router = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    let routes = axum::Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", tickets_api_router)
        .fallback_service(routes_static())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new());
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
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
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
