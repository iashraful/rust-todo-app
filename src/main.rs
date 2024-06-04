use env_logger;
use log::info;

use core::db;

pub mod core;
pub mod todo;

fn main() {
    env_logger::init();
    // Connecting to Postgres DB
    info!("Connecting to postgres.");
    db::establish_connection();
    info!("Connected.");
}
