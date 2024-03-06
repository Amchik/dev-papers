use std::fmt;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct EmptyErrorData;
impl fmt::Display for EmptyErrorData {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[allow(dead_code)]
pub enum Response<T = EmptyErrorData, E: fmt::Display + Serialize = EmptyErrorData> {
    Success(T),
    Error(Error),
    ErrorData(Error, E),
}
pub type EmptyResponse = Response<EmptyErrorData, EmptyErrorData>;

macro_rules! impl_error {
    ($(#[$a:meta])* $v:vis enum $e:ident { $( $(#[$av:meta])* $var:ident($code:literal) = (StatusCode::$scode:ident, $s:literal) ),+ $(,)? }) => {
        $(#[$a])*
        $v enum $e {
            $( $var = $code ),+
        }
        impl $e {
            #[inline(always)]
            pub const fn error_name(self) -> &'static str {
                match self {
                    $( Self::$var => stringify!($var) ),+
                }
            }
            #[inline(always)]
            pub const fn message(self) -> &'static str {
                match self {
                    $( Self::$var => $s ),+
                }
            }
            #[inline(always)]
            pub const fn http_code(self) -> StatusCode {
                match self {
                    $( Self::$var => StatusCode::$scode ),+
                }
            }
        }
    };
}

impl_error! {
    #[derive(Clone, Copy)]
    #[allow(dead_code)]
    #[repr(u32)]
    pub enum Error {
        InvalidInput(20_001) = (StatusCode::BAD_REQUEST, "Invalid data in params (query/body)"),

        NotFound(40_001) = (StatusCode::NOT_FOUND, "Object not found"),
        Conflict(40_002) = (StatusCode::CONFLICT, "New object conflicts with existing"),

        AuthorizationRequired(60_001) = (StatusCode::UNAUTHORIZED, "Endpoint requires authorization"),
        InvalidToken(60_002) = (StatusCode::UNAUTHORIZED, "Invalid or expired authorization token"),
        Forbidden(60_003) = (StatusCode::FORBIDDEN, "Not enough rights to access resource"),
        NoAccess(60_004) = (StatusCode::FORBIDDEN, "Not enough scopes to access resource"),

        Obsolete(70_001) = (StatusCode::NOT_FOUND, "Outdated API version"),
    }
}

impl<T: Serialize, E: fmt::Display + Serialize> IntoResponse for Response<T, E> {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct SuccessResponse<T> {
            ok: bool,
            result: T,
        }
        #[derive(Serialize)]
        struct FailedResponse {
            ok: bool,
            error_code: u32,
            error_name: &'static str,
            error_description: &'static str,
            error_message: Option<String>,
        }

        match self {
            Response::Success(result) => {
                let mut j = Json(SuccessResponse { ok: true, result }).into_response();
                *j.status_mut() = StatusCode::OK;
                j
            }
            Response::Error(err) => {
                let mut j = Json(FailedResponse {
                    ok: false,
                    error_code: (err as u32),
                    error_name: err.error_name(),
                    error_description: err.message(),
                    error_message: None,
                })
                .into_response();
                *j.status_mut() = err.http_code();
                j
            }
            Response::ErrorData(err, msg) => {
                let mut j = Json(FailedResponse {
                    ok: false,
                    error_code: (err as u32),
                    error_name: err.error_name(),
                    error_description: err.message(),
                    error_message: Some(format!("{msg}")),
                })
                .into_response();
                *j.status_mut() = err.http_code();
                j
            }
        }
    }
}

pub mod microservice {
    use axum::{
        async_trait,
        extract::{FromRequestParts, State},
        http::request::Parts,
    };

    use crate::routes::AppState;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum MicroserviceAuthorization {
        Telegram,
    }

    #[async_trait]
    impl FromRequestParts<AppState> for MicroserviceAuthorization {
        type Rejection = super::EmptyResponse;

        async fn from_request_parts(
            parts: &mut Parts,
            state: &AppState,
        ) -> Result<Self, Self::Rejection> {
            let State(AppState { config, .. }) =
                State::<AppState>::from_request_parts(parts, state)
                    .await
                    .expect("state should not fail");

            let token = parts
                .headers
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.split_once(' '));

            let Some((microservice, token)) = token else {
                return Err(super::EmptyResponse::Error(
                    super::Error::AuthorizationRequired,
                ));
            };

            match microservice {
                "Internal-TelegramMicroservice"
                    if config
                        .telegram
                        .as_ref()
                        .map(|v| v.shared_key == token)
                        .unwrap_or_default() =>
                {
                    Ok(Self::Telegram)
                }

                _ => Err(super::EmptyResponse::Error(
                    super::Error::AuthorizationRequired,
                )),
            }
        }
    }
}
