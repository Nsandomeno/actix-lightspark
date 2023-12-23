use crate::config;
use std::time::Duration;
use reqwest::Client;
use secrecy::{Secret, ExposeSecret};
use tracing::{error, info};

#[derive(Clone)]
pub enum PlaidMode {
    Production,
    Sandbox,
    Development,
}

/// the http request to this API
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlaidLinkPayload {
    pub phone_number   : String,
    pub client_user_id : String,
}


/// the http request to the Plaid API
#[derive(serde::Serialize)]
pub struct PlaidLinkTokenRequest {
    pub client_id    : String,
    pub secret       : String,
    // pub redirect_uri : String,
    // pub webhook      : String,
    pub client_name  : String,
    pub language     : String,
    pub country_codes: Vec<String>,
    pub products     : Vec<String>,
    pub user         : PlaidUser,
}


#[derive(serde::Serialize)]
pub struct PlaidUser {
    pub client_user_id: String,
    pub phone_number  : String,
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct PlaidLinkTokenResponse {
    pub link_token      : String,
    pub expiration      : String,
    pub request_id      : String,
    pub hosted_link_url : Option<String>,
}


#[derive(Clone)]
pub struct Plaid {
    pub client_id : Secret<String>,
    pub base_uri  : String,
    pub mode      : PlaidMode,
    pub http_client   : Client,
    pub client_secret : Secret<String>,
}


impl Plaid {
    pub fn new(config: config::Config) -> Self {
        // http client
        let http_client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .unwrap();

        Plaid { 
            client_id: config.plaid_client_id.clone(), 
            base_uri: config.plaid_base_url.clone(), 
            mode: PlaidMode::Sandbox, 
            http_client: http_client, 
            client_secret:  config.plaid_client_secret.clone()
        }
    }

    pub async fn link_token(
        &self,
        payload: PlaidLinkPayload
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/link/token/create", self.base_uri);

        let language = "en".to_string();

        let mut products = Vec::new();
        products.push("auth".to_string());

        let mut ccodes = Vec::new();
        ccodes.push("US".to_string());
        
        let request = PlaidLinkTokenRequest {
            client_name: "Fulminology Labs".to_string(),
            // redirect_uri: "https://fulminologylabs.co/inactive/redirect".to_string(),
            // webhook: "https://sample-web-hook.com".to_string(),
            client_id: self.client_id.expose_secret().clone(),
            secret: self.client_secret.expose_secret().clone(),
            country_codes: ccodes,
            products: products,
            language: language,
            user: PlaidUser {
                client_user_id: payload.client_user_id, 
                phone_number: payload.phone_number
            },
        };
        
        let res = self.http_client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await;

        match res {
            Ok(response) => {
                let status_code = response.status().as_u16();

                info!("Plaid status code: {}", status_code);
                return response.json::<serde_json::Value>().await
                
            },
            Err(err) => {
                let msg = err.to_string();
                error!("Failed to handle response... {}", msg);
                return Err(err);
            }
        }   
    }
}

