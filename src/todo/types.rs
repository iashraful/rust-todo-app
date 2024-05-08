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
