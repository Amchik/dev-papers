use axum::Router;
use dp_core::v1::endpoint;

use super::AppState;

pub mod api;
pub mod auth;
pub mod models;
pub mod projects;
pub mod users;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .nest(endpoint::auth::PREFIX, auth::get_routes())
        .nest(endpoint::user::PREFIX, users::get_routes())
        .nest(endpoint::projects::PREFIX, projects::get_routes())
}
