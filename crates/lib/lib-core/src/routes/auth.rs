use axum::{
    body::Body,
    extract::{Query, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use lib_auth::helper::{generate_password_and_jwt, verify_password_and_generate_jwt};
use lib_data::{
    auth::{LoginActions, LoginData},
    user::UserBmc,
};
use tower_cookies::cookie::time::Duration;
use tower_cookies::{cookie::SameSite, Cookie, Cookies};

use crate::{error::ApiResult, middleware::auth::AuthUser, state::AppState};

pub fn initialize_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(post_login).delete(delete_login))
        .route("/fl", post(post_first_login))
}

async fn post_login(
    State(state): State<AppState>,
    cookies: Cookies,
    Query(actions): Query<LoginActions>,
    Json(LoginData { email, password }): Json<LoginData>,
) -> ApiResult<impl IntoResponse> {
    let user = UserBmc::find_by_email(&state.mm, &email).await;

    if let Err(e) = user {
        log::debug!("User could not be fetched: {e:?}");
        return Err(StatusCode::NOT_FOUND);
    }
    let user = user.unwrap();

    if user.is_none() {
        log::debug!("User could was not found, email: {}", email);
        return Err(StatusCode::NOT_FOUND);
    }
    let user = user.unwrap();

    if !user.is_active {
        log::debug!("User is deactivated, email: {}", email);
        return Err(StatusCode::NOT_FOUND);
    }

    let claims = lib_auth::JwtCustomClaims {
        uid: user.id
    };

    let jwt = verify_password_and_generate_jwt(claims, password, user.password_hash)
        .await
        .map_err(|e| {
            log::error!("Authentication error {e:?}");
            StatusCode::NOT_FOUND
        })?;

    let _ = UserBmc::update_last_login(&state.mm, user.id).await;

    // TODO: SAVE JWT IN DATABASE TO VALIDATE IF THE TOKEN IS VALID

    let cookie = Cookie::build(("Authorization", jwt))
        .max_age(Duration::weeks(2))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    cookies.add(cookie);

    let redirect = if user.first_login {
        Some("/fl".into())
    } else {
        actions.redirect
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("REDIRECT", redirect.unwrap_or("/".into()))
        .body(Body::empty())
        .unwrap())
}

async fn delete_login(
    State(_state): State<AppState>,
    cookies: Cookies,
) -> ApiResult<impl IntoResponse> {
    // TODO: DELETE AUTH TOKEN FROM DATABASE

    let cookie = Cookie::build(("Authorization", ""))
        .max_age(Duration::milliseconds(0))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();
    cookies.add(cookie);

    Ok(())
}

async fn post_first_login(
    AuthUser(user): AuthUser,
    State(state): State<AppState>,
    Json(new_password): Json<String>,
) -> ApiResult<impl IntoResponse> {
    let (hash, _) = generate_password_and_jwt(
        lib_auth::JwtCustomClaims {
            uid: user.id
        },
        new_password,
    )
    .await
    .map_err(|e| {
        log::error!("Could not generate new hash-jwt pair: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    UserBmc::set_password_by_id(&state.mm, user.id, hash)
        .await
        .map_err(|e| {
            log::error!("Could not set new password hash - uid {}: {e:?}", user.id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    UserBmc::update_first_login(&state.mm, user.id, false)
        .await
        .map_err(|e| {
            log::error!("Could not toggle first_login flag - uid {}: {e:?}", user.id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("REDIRECT", "/")
        .body(Body::empty())
        .unwrap())
}
