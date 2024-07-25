use crate::todo::models::Todo;
use crate::todo::services::TodoService;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

pub async fn init_cli_app(pool: Pool<Manager<PgConnection>>) {
    println!("CLI is selected. Here is the menu for you.");
    println!("1. List of Todos\n2. List of Labels");

    let mut user_choice: String = String::new();
    std::io::stdin()
        .read_line(&mut user_choice)
        .expect("Invalid Choice");

    let conn = pool.get().await.expect("Error getting the connection pool");
    let mut todo_service = TodoService { conn };

    if user_choice == "1".to_string() {
        let todos: Vec<Todo> = todo_service
            .list_todos()
            .await
            .expect("Error getting todo list");
        for td in todos {
            print!("{}\r", td.title);
        }
    } else {
        println!("Invalid selection: {}", user_choice);
        let todos: Vec<Todo> = todo_service
            .list_todos()
            .await
            .expect("Error getting todo list");
        for td in todos {
            println!("{}\r", td.title);
        }
    }
}
