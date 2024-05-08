use todo::types::Todo;

pub mod todo;

fn main() {
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
