use super::database::DatabaseConfig;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub debug: bool,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub storage: StorageConfig,
    pub logging: LoggingConfig,
    pub password: PasswordConfig,
    #[serde(default)]
    pub cors: CorsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    #[serde(default = "default_jwt_expiration")]
    pub expiration: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StorageConfig {
    #[serde(default = "default_storage_path")]
    pub path: String,
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "default_log_format")]
    pub format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PasswordConfig {
    #[serde(default = "default_password_min_length")]
    pub min_length: usize,
    #[serde(default)]
    pub required_char_classes: Vec<String>,
    #[serde(default)]
    pub max_age: u32,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CorsConfig {
    #[serde(default = "default_cors_origins")]
    pub allow_origins: Vec<String>,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}
fn default_port() -> u16 {
    3000
}
fn default_base_url() -> String {
    "http://localhost:3000".to_string()
}
fn default_jwt_expiration() -> u64 {
    86400
}
fn default_storage_path() -> String {
    "./data/files".to_string()
}
fn default_max_file_size() -> u64 {
    10485760
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_log_format() -> String {
    "json".to_string()
}
fn default_password_min_length() -> usize {
    8
}
fn default_cors_origins() -> Vec<String> {
    vec!["*".to_string()]
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path =
            std::env::var("RSMINE_CONFIG_PATH").unwrap_or_else(|_| "./config.toml".to_string());

        Config::builder()
            .add_source(File::with_name(&config_path).required(false))
            .add_source(Environment::with_prefix("RSMINE").separator("__"))
            .build()?
            .try_deserialize()
    }
}
