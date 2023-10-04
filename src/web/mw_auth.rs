use crate::Result;
use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use super::AUTH_TOKEN;
use crate::Error;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let cookie = cookies.get(AUTH_TOKEN).map(|c| c.value().to_owned());
    cookie.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    let sequent = next.run(req).await;
    Ok(sequent)
}