use serde::{Deserialize, Serialize};

use crate::v1::user::User;

use super::{Endpoint, HTTPMethod};

pub const PREFIX: &'static str = "/user";

#[derive(Serialize, Deserialize)]
pub struct SelfUser {
    pub user: User,
    pub expires_at: i64,
}

pub struct GetSelf;
impl Endpoint for GetSelf {
    type Query = ();
    type Body = ();
    type Response = SelfUser;

    fn method() -> HTTPMethod {
        HTTPMethod::Get
    }
    fn partial_path() -> &'static str {
        "/@self"
    }
    fn build_path(&self) -> String {
        format!("{PREFIX}/{}", Self::partial_path())
    }
}
