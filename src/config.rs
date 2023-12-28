use secrecy::{Secret, ExposeSecret};
use serde::Deserialize;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use serde_aux::field_attributes::{deserialize_number_from_string, deserialize_bool_from_anything};
// via: https://github.com/lightsparkdev/lightspark-rs/blob/main/examples/uma-demo/src/config.rs
// thanks, zhenlu and Lightspark Eng!

/// TODO uncomment fields in this file and derive several configs
/// based on the ways to load your node's private key here:
/// https://docs.lightspark.com/lightspark-sdk/signed-operations?language=Python#signed-operations
#[derive(Debug, Clone)]
pub struct Config {
    pub log_level: String,
    pub api_client_id: String,
    pub api_client_secret: String,
    pub node_id: String,
    pub client_base_url: String,
    pub api_port: u16,
    // TODO abstract into own struct, that gets placed here
    pub plaid_base_url: String,
    pub plaid_client_id: Secret<String>,
    pub plaid_client_secret: Secret<String>,
    pub db_config: DatabaseConfig,
}

impl Config {
    pub fn new_from_env() -> Self {
        let log_level = std::env::var("LOG_LEVEL").ok();
        let api_endpoint = std::env::var("LIGHTSPARK_API_ENDPOINT").ok();
        let api_port = std::env::var("LIGHTSPARK_API_PORT").ok();
        let api_client_id = std::env::var("LIGHTSPARK_API_CLIENT_ID").ok();
        let api_client_secret = std::env::var("LIGHTSPARK_API_CLIENT_SECRET").ok();
        let node_id = std::env::var("LIGHTSPARK_NODE_ID").ok();
        // db 
        // TODO add error handling on DB config generation
        let db_config = DatabaseConfig::new();
        // plaid
        let plaid_base_url = std::env::var("PLAID_BASE_URL").ok();
        let plaid_client_id = Secret::new(
            std::env::var("PLAID_CLIENT_ID").ok().unwrap_or_default()
        );
        let plaid_client_secret = Secret::new(
            std::env::var("PLAID_CLIENT_SECRET").ok().unwrap_or_default()
        );
        Config {
            log_level: log_level.unwrap_or_default(),
            db_config: db_config,
            api_client_id: api_client_id.unwrap_or_default(),
            api_client_secret: api_client_secret.unwrap_or_default(),
            node_id: node_id.unwrap_or_default(),
            client_base_url: api_endpoint.unwrap_or_default(),
            plaid_client_id: plaid_client_id,
            plaid_base_url: plaid_base_url.unwrap_or_default(),
            plaid_client_secret: plaid_client_secret,
            api_port: api_port
                .unwrap_or("8080".to_string())
                .parse()
                .expect("Failed to parse API port.")
        }
    }
}


#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host    : String,
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port    : u16,
    pub name    : String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub use_ssl : bool,
}


impl DatabaseConfig {
    // TODO return a Result<Self, _>
    pub fn new() -> Self {
        let use_ssl = std::env::var("POSTGRES_SSL").ok().unwrap_or("false".to_string());
        let port = std::env::var("POSTGRES_PORT").ok().unwrap_or("5432".to_string());
        Self { 
            host: std::env::var("POSTGRES_HOST").ok().unwrap_or("127.0.0.1".to_string()), 
            username: std::env::var("POSTGRES_USER").ok().unwrap_or("postgres".to_string()), 
            password: Secret::new(std::env::var("POSTGRES_PASSWORD").ok().unwrap_or("password".to_string())), 
            port: port.parse::<u16>().unwrap_or(5432), 
            name: std::env::var("POSTGRES_DB").ok().unwrap_or("ls".to_string()), 
            use_ssl: use_ssl.parse::<bool>().unwrap_or(false),
        }
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db().database(&self.name);
        options.clone().log_statements(tracing_log::log::LevelFilter::Trace);
        options
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.use_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        
        PgConnectOptions::new()
        .host(&self.host)
        .username(&self.username)
        .password(self.password.expose_secret())
        .port(self.port)
        .ssl_mode(ssl_mode)
    }

}

