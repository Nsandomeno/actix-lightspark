pub mod config;
pub mod plaid;

use dotenv::dotenv;
use tracing::{info, Level};
use lightspark::key::Secp256k1SigningKey;
use tracing_subscriber::FmtSubscriber;
use lightspark::client::LightsparkClient;
use lightspark::request::auth_provider::AccountAuthProvider;
use actix_web::{App, HttpServer, web, get, post, middleware, HttpResponse, Responder};

#[get("/health-check")]
async fn health_check(
    client: web::Data<LightsparkClient<Secp256k1SigningKey>>,
) -> impl Responder {
    info!("Checking Lightspark connection...");
    let response = client.get_current_account().await; 

    match response {
        Ok(_) => {
            info!("API is healthy.");
            return HttpResponse::Ok().finish()
        },
        Err(_) => return HttpResponse::InternalServerError().finish()
    }
}

#[post("/link")]
async fn plaid_link(
    client: web::Data<plaid::Plaid>,
    payload: web::Json<plaid::PlaidLinkPayload>,
) -> impl Responder {
    info!("Creating plaid link token");
    let res = client.link_token(
        payload.0
    ).await;
    match res {
        Ok(data) => return HttpResponse::Ok().json(data),
        Err(e) => {
            println!("Failed to create plaid token with error: {}.", e.to_string());
            return HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global tracing subscriber.");

    let config = config::Config::new_from_env();
    let port = config.api_port;

    info!(config = format!("{:?}", config.clone()), "Booting server...");
    // TODO this could be wrapped as app_data if there is, to save the ammortize the cost of the client boot
    // across all client requests:
    // (1) no terrible reason to impl Clone on LightsparkClient and Requester

    let account_auth: AccountAuthProvider = AccountAuthProvider::new(
        config.api_client_id.clone(), config.api_client_secret.clone()
    );
    let client: web::Data<LightsparkClient<Secp256k1SigningKey>> = web::Data::new(LightsparkClient::new(account_auth).unwrap());
    let plaid = web::Data::new(plaid::Plaid::new(config.clone()));
    let connection_pool = web::Data::new(config::get_connection_pool(&config.db_config));
    let app_config = web::Data::new(config);
    HttpServer::new(move || {

        App::new()
            .app_data(app_config.clone())
            .app_data(client.clone())
            .app_data(plaid.clone())
            .app_data(connection_pool.clone())
            .wrap(middleware::NormalizePath::trim())
            .service(health_check)
            .service(plaid_link)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
