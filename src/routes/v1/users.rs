use axum::{routing::get, Router};
use serde::Serialize;

use crate::routes::AppState;

use super::{
    api,
    models::user::{AuthorizedUser, User},
};

pub fn get_routes() -> Router<AppState> {
    Router::new().route("/@self", get(get_self))
}

#[derive(Serialize)]
pub struct SelfUser {
    user: User,
    expires_at: i64,
}

pub async fn get_self(AuthorizedUser { user, token }: AuthorizedUser) -> api::Response<SelfUser> {
    api::Response::Success(SelfUser {
        user,
        expires_at: token.issued_at + token.ty.lifetime(),
    })
}
