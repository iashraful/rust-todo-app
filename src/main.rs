use axum;
use dotenvy::dotenv;
use log::info;
use tokio::net::TcpListener;

use api::routes::create_api_router;
use core::config::config;
use core::db;
use core::logging::LogManager;

pub mod core;
pub mod api;
pub mod todo;


#[tokio::main]
async fn main() {
    dotenv().ok();
    // Initialize the config
    let config = config().await;
    // Enable Logger
    let logger = LogManager { config };
    logger.init_logging();

    // Connecting to Postgres DB
    db::establish_connection(config.db_url().to_string());

    let host: &str = config.server_host();
    let port: u16 = config.server_port();
    let address: String = format!("{}:{}", host, port);

    let router = create_api_router();
    info!("Listening on http://{}", address);
    let listener: TcpListener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
