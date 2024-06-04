use diesel::PgConnection;
use env_logger;

use core::db;
use todo::services::TodoService;

use crate::todo::models::NewLabel;

pub mod core;
pub mod todo;

fn main() {
    // Enable logger from ENV: RUST_LOG: debug/info....
    env_logger::init();
    // Connecting to Postgres DB
    let conn: PgConnection = db::establish_connection();
    let mut todo_srv = TodoService { conn };
    let label = todo_srv.create_label(NewLabel {
        name: String::from("Test")
    });
    println!("Saved. {:?}", label)
}
