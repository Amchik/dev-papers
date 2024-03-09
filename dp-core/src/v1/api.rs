use std::fmt;

#[cfg(feature = "axum")]
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct EmptyErrorData;
impl fmt::Display for EmptyErrorData {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub enum Response<T = EmptyErrorData, E = EmptyErrorData> {
    #[serde(rename = "result")]
    Success(T),
    #[serde(rename = "error")]
    Error {
        #[serde(flatten)]
        error: Error,
        message: Option<E>,
    },
}
pub type EmptyResponse = Response<EmptyErrorData, EmptyErrorData>;

impl<T, E> Response<T, E> {
    /// Construct error with empty description
    #[inline(always)]
    pub const fn error(error: Error) -> Self {
        Response::Error {
            error,
            message: None,
        }
    }
    /// Construct error with empty description
    #[inline(always)]
    pub const fn error_description(error: Error, message: E) -> Self {
        Response::Error {
            error,
            message: Some(message),
        }
    }
}

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
            pub const fn from_code(v: u64) -> Option<Self> {
                match v {
                    $( $code => Some(Self::$var) ),+
                    ,_=>None
                }
            }
            #[cfg(feature = "axum")]
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
    #[derive(Clone, Copy, Serialize, Deserialize)]
    #[allow(dead_code)]
    #[serde(from = "ser::Error", into = "ser::Error")]
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

mod ser {
    use std::borrow::Cow;

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Error<'a> {
        code: u64,
        name: Cow<'a, str>,
        description: Cow<'a, str>,
    }
    impl<'a> From<super::Error> for Error<'a> {
        fn from(value: super::Error) -> Self {
            Self {
                code: value as u64,
                name: value.error_name().into(),
                description: value.message().into(),
            }
        }
    }
    impl<'a> From<Error<'a>> for super::Error {
        fn from(value: Error<'a>) -> Self {
            Self::from_code(value.code).expect("invalid error code")
        }
    }
}

#[cfg(feature = "axum")]
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
            // Response::error(err) => {
            //     let mut j = Json(FailedResponse {
            //         ok: false,
            //         error_code: (err as u32),
            //         error_name: err.error_name(),
            //         error_description: err.message(),
            //         error_message: None,
            //     })
            //     .into_response();
            //     *j.status_mut() = err.http_code();
            //     j
            // }
            Response::Error { error, message } => {
                let mut j = Json(FailedResponse {
                    ok: false,
                    error_code: (error as u32),
                    error_name: error.error_name(),
                    error_description: error.message(),
                    error_message: message.map(|v| format!("{v}")),
                })
                .into_response();
                *j.status_mut() = error.http_code();
                j
            }
        }
    }
}
