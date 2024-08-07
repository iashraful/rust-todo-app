use axum;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use tokio::net::TcpListener;

use api::routes::create_api_router;
use core::config::config;
use core::db;
use core::logging::LogManager;

pub mod api;
pub mod core;
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
    let conn_pool: Pool<Manager<PgConnection>> =
        db::establish_connection(config.db_url().to_string());

    let host: &str = config.server_host();
    let port: u16 = config.server_port();
    let address: String = format!("{}:{}", host, port);

    let router: axum::Router = create_api_router(conn_pool);
    println!("Listening on http://{}", address);
    let listener: TcpListener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
