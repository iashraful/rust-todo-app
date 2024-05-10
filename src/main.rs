use core::db;
use todo::types::Todo;

pub mod core;
pub mod todo;

fn main() {
    // Connecting to Postgres DB
    println!("Connecting to postgres.");
    db::establish_connection();
    println!("Connected.");

    let todo = Todo {
        id: 1,
        title: String::from("Title"),
        description: String::from("Description"),
        label: None,
        is_checked: false,
        created_at: None,
        updated_at: None,
    };
    println!("{:?}", todo);
}
