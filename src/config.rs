use dotenv::dotenv;
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
    //pub username: String,
    //pub user_id: String,
    //pub uma_encryption_private_key_hex: String,
    //pub uma_signing_private_key_hex: String,
    // pub node_master_seed_hex: String,
    pub client_base_url: String,
    pub api_port: u16,
}

impl Config {
    pub fn new_from_env() -> Self {
        dotenv().ok();

        let log_level = std::env::var("LOG_LEVEL").ok();
        let api_endpoint = std::env::var("LIGHTSPARK_API_ENDPOINT").ok();
        let api_port = std::env::var("LIGHTSPARK_API_PORT").ok();
        let api_client_id = std::env::var("LIGHTSPARK_API_CLIENT_ID").ok();
        let api_client_secret = std::env::var("LIGHTSPARK_API_CLIENT_SECRET").ok();
        //let master_seed_hex = std::env::var("LIGHTSPARK_MASTER_SEED_HEX").ok();
        let node_id = std::env::var("LIGHTSPARK_NODE_ID").ok();
        //let username = std::env::var("LIGHTSPARK_USERNAME").ok();
        //let user_id = std::env::var("LIGHTSPARK_USER_ID").ok();
        // let uma_encryption_private_key_hex =
        //     std::env::var("LIGHTSPARK_UMA_ENCRYPTION_PRIVATE_KEY_HEX").ok();
        // let uma_signing_private_key_hex =
        //     std::env::var("LIGHTSPARK_UMA_SIGNING_PRIVATE_KEY_HEX").ok();

        Config {
            // TODO consider use of unwrap_or_default, doesn't look like their are defaults...

            log_level: log_level.unwrap_or_default(),
            api_client_id: api_client_id.unwrap_or_default(),
            api_client_secret: api_client_secret.unwrap_or_default(),
            node_id: node_id.unwrap_or_default(),
            //username: username.unwrap_or_default(),
            //user_id: user_id.unwrap_or_default(),
            //uma_encryption_private_key_hex: uma_encryption_private_key_hex.unwrap_or_default(),
            //uma_signing_private_key_hex: uma_signing_private_key_hex.unwrap_or_default(),
            //node_master_seed_hex: master_seed_hex.unwrap_or_default(),
            client_base_url: api_endpoint.unwrap_or_default(),
            api_port: api_port
                .unwrap_or("8080".to_string())
                .parse()
                .expect("Failed to parse API port.")
        }
    }
}

