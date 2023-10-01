use std::net::SocketAddr;

use axum;
use axum::response::Html;
use axum::routing;
use tokio;

#[tokio::main]
async fn main() {
    let routes = axum::Router::new().route(
        "/hello",
        routing::get(|| async { Html("<p>Hello, world!</p>") }),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 8095));
    println!("->> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("Starting a server should work.");
}
