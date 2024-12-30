use std::fs;

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    base_path: String
}

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_content = fs::read_to_string("config/default.toml")
        .expect("Failed to read config");

    toml::from_str(&config_content)
        .expect("Failed to parse config")
});

impl Config {
    pub fn base_path() -> &'static str {
        &CONFIG.base_path
    }
}
