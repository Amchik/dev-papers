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

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Error {
    InvalidInput,

    NotFound,
    Conflict,

    AuthorizationRequired,
    InvalidToken,

    Obsolete,
}

impl Error {
    pub const fn message(self) -> &'static str {
        match self {
            Error::InvalidInput => "Invalid data in params (query/body)",
            Error::NotFound => "Object not found",
            Error::Conflict => "New object conflicts with existing",
            Error::AuthorizationRequired => "Endpoint requires authorization",
            Error::InvalidToken => "Invalid authorization token",
            Error::Obsolete => "API version obsolote",
        }
    }
    pub const fn http_code(self) -> StatusCode {
        match self {
            Error::InvalidInput => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::Conflict => StatusCode::CONFLICT,
            Error::AuthorizationRequired => StatusCode::UNAUTHORIZED,
            Error::InvalidToken => StatusCode::UNAUTHORIZED,
            Error::Obsolete => StatusCode::NOT_FOUND,
        }
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
            error_description: String,
        }

        match self {
            Response::Success(result) => {
                let mut j = Json(SuccessResponse { ok: true, result }).into_response();
                *j.status_mut() = StatusCode::OK;
                j
            }
            Response::Error(err) => {
                let mut j = Json(FailedResponse {
                    ok: true,
                    error_description: err.message().to_owned(),
                })
                .into_response();
                *j.status_mut() = err.http_code();
                j
            }
            Response::ErrorData(err, msg) => {
                let mut j = Json(FailedResponse {
                    ok: false,
                    error_description: format!("{}: {msg}", err.message()),
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
