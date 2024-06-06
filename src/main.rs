use axum;
use axum::Router;
use axum::routing::get;
use dotenvy::dotenv;
use log::{debug, info};
use tokio::net::TcpListener;

use core::config::config;
use core::db;
use core::logging::LogManager;

pub mod core;
pub mod todo;

async fn index() -> &'static str {
    debug!("Request Received.");
    "OK!"
}

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

    let app = Router::new()
        .route("/", get(index));
    info!("Listening on http://{}", address);
    let listener: TcpListener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
