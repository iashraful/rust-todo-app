use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;

use api::server::run_server;
use clap::Parser;
use cli::app::init_cli_app;
use cli::args;
use core::config::config;
use core::db;
use core::enums::AppMode;
use core::logging::LogManager;
use log::info;

pub mod api;
pub mod cli;
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

    // Parse the args here to operate the CLI or API
    let arguments = args::Args::parse();

    info!("Selected Mode: {}", arguments.mode);
    if arguments.mode == AppMode::API.to_string() {
        run_server(config, conn_pool).await;
    } else if arguments.mode == AppMode::CLI.to_string() {
        // Do the CLI things here.
        init_cli_app(conn_pool).await;
    } else {
        panic!("Unknown command for --mode or -m. Avaiable: API, CLI")
    }
}
