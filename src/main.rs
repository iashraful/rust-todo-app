use todo::types::Todo;

pub mod todo;

fn main() {
    let todo = Todo{
        id: 1,
        title: String::from("Title"),
        description: String::from("Description"),
        is_checked: false,
        created_at: String::from("Date")
    };
    println!("{:?}", todo);
    todo.save();
}
