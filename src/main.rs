use diesel::PgConnection;
use env_logger;

use core::db;
use todo::services::TodoService;

use crate::todo::models::{NewLabel, Label};

pub mod core;
pub mod todo;

fn main() {
    // Enable logger from ENV: RUST_LOG: debug/info....
    env_logger::init();
    // Connecting to Postgres DB
    let conn: PgConnection = db::establish_connection();
    let mut todo_srv = TodoService { conn };
    let label: Label = todo_srv.create_label(NewLabel {
        name: String::from("Test")
    }).expect("Error creating label.");
    println!("Saved. {:?}", label)
}
