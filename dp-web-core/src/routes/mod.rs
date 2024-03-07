use sqlx::SqlitePool;

use crate::config::Config;

pub mod v1;

#[derive(Clone)]
pub struct AppState {
    pub config: &'static Config,
    pub db: SqlitePool,
}
