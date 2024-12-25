use axum::{
    body::Body,
    extract::Request,
    handler::Handler,
    response::{IntoResponse, Redirect},
    Router,
};
use tower::ServiceExt;
use tower_cookies::CookieManagerLayer;
use tower_http::services::{ServeDir, ServeFile};

use crate::{middleware::auth::AuthUser, state};

mod auth;

pub fn initialize(app_state: state::AppState) -> Router {
    let serve_dir = if std::env::var("CARGO").is_ok() {
        log::debug!("Running in debug mode");
        "target/public"
    } else {
        log::debug!("Running in production mode");
        "public"
    };

    let app_service = move |auth: Option<AuthUser>, req: Request<Body>| async move {
        let uri = req.uri().to_string();

        if auth.is_none()
            && uri != "/login"
            && uri != "/fl"
            && !uri.starts_with("/assets")
        {
            return Redirect::permanent("/login").into_response();
        }

        let res = match &uri {
            s if s.contains(".") => ServeDir::new(serve_dir).oneshot(req).await,
            _ => {
                ServeFile::new(format!("{serve_dir}/index.html"))
                    .oneshot(req)
                    .await
            }
        };

        res.into_response()
    };

    Router::new()
        .nest("/api/login", auth::initialize_routes())
        .fallback_service(app_service.with_state(app_state.clone()))
        .layer(CookieManagerLayer::new())
        .with_state(app_state)
}
