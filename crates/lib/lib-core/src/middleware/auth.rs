use axum::{
    async_trait,
    body::Body,
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts, StatusCode},
    response::Response,
    RequestPartsExt,
};
use lib_auth::verify_jwt;
use lib_data::user::{PartialUser, UserBmc};
use tower_cookies::{Cookie, Cookies};

use crate::state::AppState;

pub struct AuthUser(pub PartialUser);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        log::debug!("Authorizing request");

        let cookies = req.extract::<Cookies>().await.map_err(|e| {
            log::error!("Failed to get Cookies from authorization extractor: {e:#?}");
            Response::builder().status(404).body(Body::empty()).unwrap()
        })?;

        let token = cookies
            .get("Authorization")
            .map(|cookie| cookie.value().to_string());

        let token = token.ok_or_else(|| {
            cookies.remove(Cookie::from("Authorization"));
            Response::builder().status(404).body(Body::empty()).unwrap()
        })?;

        let uid = verify_jwt(&token);

        if let Ok(uid) = uid {
            let user = UserBmc::find_by_id(&app_state.mm, uid).await.map_err(|_| {
                log::debug!("Authorization failed: User not found - error: {uid}");
                Response::builder().status(404).body(Body::empty()).unwrap()
            })?;

            if let Some(user) = user {
                if user.is_active {
                    let _ = UserBmc::update_last_login(&app_state.mm, user.id).await;

                    log::debug!("Authorization results: Authorized user - id: {}", uid);

                    if user.first_login && !req.uri.path().ends_with("/fl") {
                        return Err(Response::builder()
                            .status(StatusCode::FOUND)
                            .header(header::LOCATION, "/fl")
                            .body(Body::empty())
                            .unwrap());
                    }

                    return Ok(AuthUser(user.into()));
                }
            }

            log::debug!("Authorization failed: User not found - error: {uid}");
            Err(Response::builder().status(404).body(Body::empty()).unwrap())
        } else {
            Err(Response::builder().status(404).body(Body::empty()).unwrap())
        }
    }
}
