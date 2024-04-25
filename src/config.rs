use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<Config>,
}
