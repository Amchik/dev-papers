//! # `dp-web` core library
//!
//! Core library of all API endpoints (with implementations).

use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

pub mod config;
pub mod routes;

pub async fn apply_migrations(db: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(concat!(include_str!("migrations/0001-initial.sql"),))
        .execute(db)
        .await
}
