use bitflags::bitflags;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;

use crate::v1::generic::define_types;

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

/// User
#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub ty: UserTy,
    pub username: String,
    pub telegram_id: i64,
}

/// Checks if user name is correct
/// # Example
/// ```
/// #use dp_core::v1::user::check_username;
/// assert_eq!(check_username("totally-valid-username", true));
/// assert_eq!(check_username("not valid username", false));
/// ```
pub fn check_username(v: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Za-z0-9.]+").unwrap());
    RE.is_match(v)
}
