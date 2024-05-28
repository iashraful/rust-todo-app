use core::db;

pub mod core;
pub mod todo;

fn main() {
    // Connecting to Postgres DB
    println!("Connecting to postgres.");
    db::establish_connection();
    println!("Connected.");
}
