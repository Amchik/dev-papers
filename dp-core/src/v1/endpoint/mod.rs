//! Endpoints models

pub mod auth;
pub mod projects;
pub mod user;

/// HTTP method of endpoint
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HTTPMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

/// Describes an API endpoint
pub trait Endpoint {
    type Query;
    type Body;
    type Response;

    /// HTTP method of endpoint
    fn method() -> HTTPMethod;

    /// Partial path in current group. See [`Endpoint::build_path`]
    /// for examples
    fn partial_path() -> &'static str;

    /// Full path for this configuration
    ///
    /// # Examples
    /// | [`Endpoint::partial_path`] | [`Endpoint::build_path`] |
    /// |----------------|--------------|
    /// | `/:id/pdf`     | `/paper/14/pdf` |
    /// | `/`            | `/user/`     |
    fn build_path(&self) -> String;
}
