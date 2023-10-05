use crate::{ctx::Ctx, Error, Result};
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use super::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let cookie = cookies.get(AUTH_TOKEN).map(|c| c.value().to_owned());
    let cookie = cookie.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    let (_, _, _) = parse_token(&cookie)?;
    let sequent = next.run(req).await;
    Ok(sequent)
}

fn parse_token(token: &str) -> Result<(u64, String, String)> {
    let regex_outcome = regex_captures!(r#"^user-([0-9]+)\.(.+)\.(.+)$"#, token);
    let (_whole, id, exp_date, signature) = regex_outcome.ok_or(Error::AuthFailTokenWrongFormat)?;
    id.parse::<u64>()
        .map_err(|_| Error::AuthFailTokenWrongFormat)
        .map(|id| (id, exp_date.to_owned(), signature.to_owned()))
}

#[async_trait]
impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let cookie = cookies.get(AUTH_TOKEN).map(|c| c.value().to_owned());
        let cookie = cookie.ok_or(Error::AuthFailNoAuthTokenCookie)?;
        let regex_outcome = regex_captures!(r#"^user-([0-9]+)\.(.+)\.(.+)$"#, &cookie);
        let (_whole, id, exp_date, signature) =
            regex_outcome.ok_or(Error::AuthFailTokenWrongFormat)?;
        let (user_id, _expiration_date, _signature) = id
            .parse::<u64>()
            .map_err(|_| Error::AuthFailTokenWrongFormat)
            .map(|id| (id, exp_date.to_owned(), signature.to_owned()))?;
        Ok(Ctx::new(user_id))
    }
}
