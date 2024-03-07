use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub telegram: Option<TelegramConfig>,

    pub papers_path: String,
}

#[derive(Clone, Deserialize)]
pub struct TelegramConfig {
    pub shared_key: String,
}
