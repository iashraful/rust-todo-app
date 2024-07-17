use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;

use api::server::run_server;
use clap::Parser;
use cli::args;
use core::config::config;
use core::db;
use core::logging::LogManager;

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

    println!("Selected Mode: {}", arguments.mode);
    if arguments.mode == "api".to_string() {
        run_server(config, conn_pool).await;
    } else {
        // Do the CLI things here.
        println!("CLI is selected. Here is the menu for you.");
        println!("1. List of Todos\n2. List of Labels")
    }
}
