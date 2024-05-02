use cli::Todo;

pub mod cli;


fn main() {
    let todo = Todo{
        id: 1,
        title: String::from("Title"),
        description: String::from("Description"),
        is_checked: false,
        created_at: String::from("Date")
    };
    println!("Hello, world!");
    println!("{:?}", todo);
}
