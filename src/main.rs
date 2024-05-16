use core::db;

pub mod core;
pub mod todo;
pub mod schema;

use todo::models::{Todo, Label};

fn main() {
    // Connecting to Postgres DB
    println!("Connecting to postgres.");
    db::establish_connection();
    println!("Connected.");
}
