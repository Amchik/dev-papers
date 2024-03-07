use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::request::Parts,
};
use dp_core::v1::user::{User, UserToken, UserTokenScope, UserTokenTy, UserTy};
use rand::Rng;

use crate::routes::{v1::api, AppState};

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
