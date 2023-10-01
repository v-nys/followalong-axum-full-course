use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::{Result, Error};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username == "demo1" && payload.pwd == "welcome" {
        Ok(Json(json!({
            "result": { "success": true }
        })))
    }
    else {
        Err(Error::LoginFail)
    }
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}