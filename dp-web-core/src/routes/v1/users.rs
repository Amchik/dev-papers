use axum::{routing::get, Router};
use dp_core::v1::{
    api,
    endpoint::{
        user::{GetSelf, SelfUser},
        Endpoint,
    },
};

use crate::routes::AppState;

use super::models::user::AuthorizedUser;

pub fn get_routes() -> Router<AppState> {
    Router::new().route(GetSelf::partial_path(), get(get_self))
}

pub async fn get_self(
    AuthorizedUser { user, token }: AuthorizedUser,
) -> api::Response<<GetSelf as Endpoint>::Response> {
    api::Response::Success(SelfUser {
        user,
        expires_at: token.issued_at + token.ty.lifetime(),
    })
}
