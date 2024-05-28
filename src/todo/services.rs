use crate::todo::models::Todo;

pub struct TodoService {}

impl TodoService {
    fn create(todo: Todo) {
        println!("Creating the task to the db.")
    }
}