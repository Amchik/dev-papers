use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Json, Query, State},
    routing::{post, put},
    Router,
};
use dp_core::v1::user::{check_username, User, UserToken, UserTokenTy};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::routes::{v1::models::user::generate_token, AppState};

use super::{
    api::{self, microservice::MicroserviceAuthorization},
    models::user::AuthorizedUser,
};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/telegram", put(telegram_issue_token))
        .route("/telegram", post(telegram_activate_token))
        .route("/invite", post(claim_invite_user))
        .route("/telegram/invite", post(claim_invite_telegram))
}

#[derive(Deserialize)]
pub struct IssueUserTokenQuery {
    pub telegram_id: i64,
}

#[derive(Serialize)]
pub struct IssueUserTokenResponse {
    pub issued_at: i64,
    pub expires_in: i64,
    pub user_id: i64,
    pub token: String,
    pub ty: UserTokenTy,
}

#[derive(Deserialize)]
pub struct ClaimInviteBody {
    pub invite: String,
    pub username: String,
    pub telegram_id: i64,
}

pub async fn claim_invite(
    invite: String,
    username: String,
    telegram_id: i64,
    db: &Pool<Sqlite>,
) -> Result<i64, api::Error> {
    if !check_username(&username) || telegram_id < 0 {
        return Err(api::Error::InvalidInput);
    }

    let inv = sqlx::query!(
        "select id,user_ty from userinvite where invite = ? and claimed_user_id is null",
        invite
    )
    .fetch_one(db)
    .await
    .ok()
    .map(|v| (v.id, v.user_ty));
    let Some((invite_id, user_ty)) = inv else {
        return Err(api::Error::NotFound);
    };

    let res = sqlx::query!(
        "insert into user(ty,username,telegram_id) values (?,?,?)",
        user_ty,
        username,
        telegram_id
    )
    .execute(db)
    .await;

    let user_id = match res {
        Ok(v) => v.last_insert_rowid(),
        Err(_) => return Err(api::Error::Conflict),
    };

    sqlx::query!(
        "update userinvite set claimed_user_id = ? where id = ?",
        user_id,
        invite_id
    )
    .execute(db)
    .await
    .expect("update userinvite");

    Ok(user_id)
}

pub async fn issue_token(
    user_id: i64,
    ty: UserTokenTy,
    db: &Pool<Sqlite>,
) -> IssueUserTokenResponse {
    let token = generate_token();
    let issued_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let expires_in = issued_at + ty.lifetime();

    let ity = ty as i64;
    sqlx::query!(
        "insert into usertoken(user_id,token,issued_at,ty) values (?,?,?,?)",
        user_id,
        token,
        issued_at,
        ity
    )
    .execute(db)
    .await
    .expect("insert user token from telegram activation");

    IssueUserTokenResponse {
        issued_at,
        expires_in,
        user_id,
        token,
        ty,
    }
}

pub async fn claim_invite_user(
    State(AppState { db, .. }): State<AppState>,
    Json(ClaimInviteBody {
        invite,
        username,
        telegram_id,
    }): Json<ClaimInviteBody>,
) -> api::Response<IssueUserTokenResponse> {
    let user_id = match claim_invite(invite, username, telegram_id, &db).await {
        Ok(v) => v,
        Err(e) => return api::Response::Error(e),
    };

    api::Response::Success(issue_token(user_id, UserTokenTy::UserLimited, &db).await)
}

pub async fn claim_invite_telegram(
    ms: MicroserviceAuthorization,
    State(AppState { db, .. }): State<AppState>,
    Json(ClaimInviteBody {
        invite,
        username,
        telegram_id,
    }): Json<ClaimInviteBody>,
) -> api::Response<IssueUserTokenResponse> {
    if !matches!(ms, MicroserviceAuthorization::Telegram) {
        return api::Response::Error(api::Error::AuthorizationRequired);
    }

    let user_id = match claim_invite(invite, username, telegram_id, &db).await {
        Ok(v) => v,
        Err(e) => return api::Response::Error(e),
    };

    api::Response::Success(issue_token(user_id, UserTokenTy::TelegramAuthorization, &db).await)
}

pub async fn telegram_activate_token(
    AuthorizedUser {
        user: User { id: user_id, .. },
        token: UserToken {
            id: token_id, ty, ..
        },
    }: AuthorizedUser,
    State(AppState { db, .. }): State<AppState>,
) -> api::Response<IssueUserTokenResponse> {
    if !matches!(ty, UserTokenTy::TelegramAuthorization) {
        return api::Response::Error(api::Error::AuthorizationRequired);
    }

    sqlx::query!("delete from usertoken where id = ?", token_id)
        .execute(&db)
        .await
        .expect("delete user token from telegram activation");

    api::Response::Success(issue_token(user_id, UserTokenTy::UserLimited, &db).await)
}

pub async fn telegram_issue_token(
    ms: MicroserviceAuthorization,
    Query(IssueUserTokenQuery { telegram_id }): Query<IssueUserTokenQuery>,
    State(AppState { db, .. }): State<AppState>,
) -> api::Response<IssueUserTokenResponse> {
    if !matches!(ms, MicroserviceAuthorization::Telegram) {
        return api::Response::Error(api::Error::AuthorizationRequired);
    }

    let user_id = sqlx::query!("select id from user where telegram_id = ?;", telegram_id)
        .fetch_one(&db)
        .await
        .map(|v| v.id);

    let Ok(user_id) = user_id else {
        return api::Response::Error(api::Error::NotFound);
    };

    api::Response::Success(issue_token(user_id, UserTokenTy::TelegramAuthorization, &db).await)
}
