use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};
use crate::{Result, Error};

use super::AUTH_TOKEN;

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}



async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username == "demo1" && payload.pwd == "welcome" {
        cookies.add(Cookie::new(AUTH_TOKEN,"user-1.expdate.signature"));
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