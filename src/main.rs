use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::Router;
use clap::{Parser, Subcommand};
use config::Config;
use routes::v1::models::user::generate_token;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

use crate::routes::AppState;

mod config;
mod routes;

#[derive(Parser)]
#[command(version, about = "API server for hosting papers", long_about = None, arg_required_else_help = true)]
struct Args {
    /// Path which stores the papers
    #[arg(short, long, default_value = "config.yml")]
    config: PathBuf,

    /// Path to sqlite database
    #[arg(short, long, default_value = "papers.sqlite")]
    database: String,

    #[command(subcommand)]
    subcommand: Subcommands,
}
#[derive(Subcommand)]
enum Subcommands {
    /// Start the web service
    Start {
        /// Port to listen
        #[arg(short, long, default_value = "0.0.0.0:3000")]
        ip: String,
    },
    /// Issue user invite
    CreateInvite {
        /// Invite reason
        #[arg(long)]
        reason: String,

        /// Type of user
        #[arg(long, default_value = routes::v1::models::user::UserTy::Unregistered)]
        user_type: routes::v1::models::user::UserTy,
    },
}

async fn apply_migrations(db: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(concat!(include_str!("migrations/0001-initial.sql"),))
        .execute(db)
        .await
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let Ok(db) = SqlitePool::connect(&args.database).await else {
        panic!("Failed to connect to database");
    };
    if let Err(e) = apply_migrations(&db).await {
        panic!("Failed to apply migrations: {e}");
    }

    let cfg: Config = match fs::read_to_string(&args.config).map(|v| serde_yaml::from_str(&v)) {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => panic!(
            "Failed to parse yaml config file '{}': {e}",
            args.config.to_string_lossy()
        ),
        Err(e) => panic!(
            "Failed to read config file '{}': {e}",
            args.config.to_string_lossy()
        ),
    };

    match args.subcommand {
        Subcommands::Start { ip } => {
            let app = Router::new()
                .nest("/v1", routes::v1::get_routes())
                .with_state(AppState {
                    config: Box::leak(Box::new(cfg)),
                    db,
                });

            println!("Server starting at {ip}");
            let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
        Subcommands::CreateInvite { reason, user_type } => {
            let token = generate_token();
            let issued_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            let user_type = user_type as i64;
            let res = sqlx::query!(
                "insert into userinvite(user_ty,reason,invite,issued_at) values (?,?,?,?)",
                user_type,
                reason,
                token,
                issued_at
            )
            .execute(&db)
            .await;
            match res {
                Ok(_) => println!("Invite token: {token}"),
                Err(e) => panic!("Failed to insert to database: {e}"),
            }
        }
    }
}
