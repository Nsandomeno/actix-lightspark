pub mod config;

use lightspark::key::Secp256k1SigningKey;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use lightspark::client::LightsparkClient;
use lightspark::request::auth_provider::AccountAuthProvider;
use actix_web::{App, HttpServer, web, get, middleware, HttpResponse, Responder};


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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    let client: LightsparkClient<Secp256k1SigningKey> = LightsparkClient::new(account_auth).unwrap();

    HttpServer::new(move || {

        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(client.clone()))
            .wrap(middleware::NormalizePath::trim())
            .service(health_check)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
