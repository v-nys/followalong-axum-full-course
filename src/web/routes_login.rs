use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};
use crate::{Result, Error};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

const AUTH_TOKEN: &str = "auth_token";

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username == "demo1" && payload.pwd == "welcome" {
        cookies.add(Cookie::new(AUTH_TOKEN,"username.expdate.signature"));
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