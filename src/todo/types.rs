#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub is_checked: bool,
    pub created_at: String,
}

impl Todo {
    pub fn save(&self) {
        println!("Saving the todo.")
    }
}
