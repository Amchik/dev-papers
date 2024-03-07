use axum::Router;

use super::AppState;

pub mod api;
pub mod auth;
pub mod models;
pub mod projects;
pub mod users;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::get_routes())
        .nest("/user", users::get_routes())
        .nest("/projects", projects::get_routes())
}
