use axum::{routing::get, Json, Router};
use lib_data::user::UserMeData;

use crate::{error::ApiResult, middleware::auth::AuthUser, state::AppState};

pub fn initialize_routes() -> Router<AppState> {
    Router::new().route("/me", get(get_me))
}

async fn get_me(AuthUser(user): AuthUser) -> ApiResult<Json<UserMeData>> {
    Ok(Json(user.into()))
}
