use axum;
use axum::Router;
use axum::routing::get;
use dotenvy::dotenv;
use env_logger;
use log::info;
use tokio::net::TcpListener;

use core::config::config;
use core::db;

pub mod core;
pub mod todo;

async fn index() -> &'static str {
    "Hello!"
}

#[tokio::main]
async fn main() {
    // Enable logger from ENV: RUST_LOG: debug/info....
    dotenv().ok();
    env_logger::init();

    // Initialize the config
    let config = config().await;

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
