use serde::{Deserialize, Serialize};

use crate::v1::user::UserTokenTy;

use super::{Endpoint, HTTPMethod};

pub const PREFIX: &'static str = "/auth";

#[derive(Serialize, Deserialize)]
pub struct IssueUserTokenQuery {
    pub telegram_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct IssueUserTokenResponse {
    pub issued_at: i64,
    pub expires_in: i64,
    pub user_id: i64,
    pub token: String,
    pub ty: UserTokenTy,
}

#[derive(Serialize, Deserialize)]
pub struct ClaimInviteBody {
    pub invite: String,
    pub username: String,
    pub telegram_id: i64,
}

pub struct TelegramIssueToken;
impl Endpoint for TelegramIssueToken {
    type Body = ();
    type Query = IssueUserTokenQuery;
    type Response = IssueUserTokenResponse;
    fn method() -> HTTPMethod {
        HTTPMethod::Put
    }
    fn partial_path() -> &'static str {
        "/telegram"
    }
    fn build_path(&self) -> String {
        format!("{PREFIX}{}", Self::partial_path())
    }
}
pub struct TelegramActivateToken;
impl Endpoint for TelegramActivateToken {
    type Body = ();
    type Query = ();
    type Response = IssueUserTokenResponse;
    fn method() -> HTTPMethod {
        HTTPMethod::Post
    }
    fn partial_path() -> &'static str {
        "/telegram"
    }
    fn build_path(&self) -> String {
        format!("{PREFIX}{}", Self::partial_path())
    }
}

pub struct ClaimInviteUser;
impl Endpoint for ClaimInviteUser {
    type Body = ClaimInviteBody;
    type Query = ();
    type Response = IssueUserTokenResponse;

    fn method() -> HTTPMethod {
        HTTPMethod::Post
    }
    fn partial_path() -> &'static str {
        "/invite"
    }
    fn build_path(&self) -> String {
        format!("{PREFIX}{}", Self::partial_path())
    }
}
pub struct ClaimInviteTelegram;
impl Endpoint for ClaimInviteTelegram {
    type Body = ClaimInviteBody;
    type Query = ();
    type Response = IssueUserTokenResponse;

    fn method() -> HTTPMethod {
        HTTPMethod::Post
    }
    fn partial_path() -> &'static str {
        "/telegram/invite"
    }
    fn build_path(&self) -> String {
        format!("{PREFIX}{}", Self::partial_path())
    }
}
