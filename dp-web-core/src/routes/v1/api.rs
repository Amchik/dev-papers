pub mod microservice {
    use axum::{
        async_trait,
        extract::{FromRequestParts, State},
        http::request::Parts,
    };
    use dp_core::v1::api;

    use crate::routes::AppState;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum MicroserviceAuthorization {
        Telegram,
    }

    #[async_trait]
    impl FromRequestParts<AppState> for MicroserviceAuthorization {
        type Rejection = api::EmptyResponse;

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
                return Err(api::EmptyResponse::Error(api::Error::AuthorizationRequired));
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

                _ => Err(api::EmptyResponse::Error(api::Error::AuthorizationRequired)),
            }
        }
    }
}
