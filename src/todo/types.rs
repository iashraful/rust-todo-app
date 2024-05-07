use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Label {
    pub id: u32,
    pub name: String,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}


#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub label: Option<Label>,
    pub is_checked: bool,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}

impl Todo {
    pub fn save(&mut self) {
        println!("Saving the todo.");
        if self.created_at == None {
            self.created_at = Some(Local::now())
        }
        println!("{:?}", self)
    }
}
