use std::sync::Arc;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}


#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<Config>,
}