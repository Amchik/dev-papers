use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::request::Parts,
};
use bitflags::bitflags;
use clap::{
    builder::{OsStr, PossibleValue},
    ValueEnum,
};
use once_cell::sync::Lazy;
use rand::Rng;
use regex::Regex;
use serde::Serialize;

use crate::{
    define_types,
    routes::{v1::api, AppState},
};

define_types! {
    /// Type of user
    #[derive(Serialize, Copy, Clone, PartialEq, Eq)]
    pub enum UserTy: i64 {
        /// User that should confirm registration
        Unregistered = 0,

        /// User that should be confirmed by moderators
        Unverified = 1,

        /// Normal user
        Normal = 2,
    }
}

impl ValueEnum for UserTy {
    fn value_variants<'a>() -> &'a [Self] {
        Self::ALL_VALUES
    }
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.as_str()))
    }
}
impl Into<OsStr> for UserTy {
    fn into(self) -> OsStr {
        OsStr::from(self.as_str())
    }
}

define_types! {
    /// Type of user token
    #[derive(Serialize, Copy, Clone, PartialEq, Eq)]
    pub enum UserTokenTy: i64 {
        UserLimited = 0,
        TelegramAuthorization = 1,
    }
}

impl UserTokenTy {
    /// Returns how long token lives in milliseconds.
    pub const fn lifetime(self) -> i64 {
        match self {
            Self::UserLimited => 999999999,
            Self::TelegramAuthorization => 20 * 60 * 1000,
        }
    }
}

/// Scopes of user token
#[derive(Serialize)]
pub struct UserTokenScope(i64);

bitflags! {
    impl UserTokenScope: i64 {
        const _ = !0;
    }
}

/// User token
#[derive(Serialize)]
pub struct UserToken {
    pub id: i64,
    pub ty: UserTokenTy,
    pub user_id: i64,
    pub scope: UserTokenScope,
    pub issued_at: i64,

    pub token: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub ty: UserTy,
    pub username: String,
    pub telegram_id: i64,
}

pub struct AuthorizedUser {
    pub user: User,
    pub token: UserToken,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthorizedUser {
    type Rejection = api::EmptyResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let State(AppState { db, .. }) = State::<AppState>::from_request_parts(parts, state)
            .await
            .expect("state should not fail");

        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .and_then(|v| v.split_once(':'))
            .map(|(i, t)| (i.parse::<i64>().ok(), t));

        let Some((Some(user_id), token)) = token else {
            return Err(api::EmptyResponse::Error(api::Error::AuthorizationRequired));
        };

        let res = sqlx::query!(
            r#"select usertoken.*, user.ty as userty, user.username as username, user.telegram_id as telegram_id
                from usertoken
                join user on usertoken.user_id = user.id
                where usertoken.token = ? and user.id = ?;"#,
            token,
            user_id
        )
        .fetch_one(&db)
        .await
        .map_err(|_| api::EmptyResponse::Error(api::Error::InvalidToken))?;

        let token_ty = UserTokenTy::from_bits(res.ty);

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;

        if token_ty.lifetime() + res.issued_at < time {
            _ = sqlx::query!("delete from usertoken where id = ?", res.id)
                .execute(&db)
                .await;
            return Err(api::EmptyResponse::Error(api::Error::InvalidToken));
        }

        Ok(Self {
            token: UserToken {
                id: res.id,
                ty: token_ty,
                user_id,
                issued_at: res.issued_at,
                scope: UserTokenScope::from_bits_retain(res.scope),
                token: token.to_owned(),
            },
            user: User {
                id: user_id,
                ty: UserTy::from_bits(res.userty),
                username: res.username,
                telegram_id: res.telegram_id,
            },
        })
    }
}

pub fn generate_token() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

    let mut rng = rand::thread_rng();

    let token: String = (0..48)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    token
}

pub fn check_username(v: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Za-z0-9.]+").unwrap());
    RE.is_match(v)
}
