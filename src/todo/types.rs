use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BaseModel {
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}

impl BaseModel {
    fn json(&mut self) {
        if self.created_at == None {
            self.created_at = Some(Local::now())
        }
        if self.updated_at == None {
            self.updated_at = Some(Local::now())
        }
        serde_json::to_string(&self).unwrap();
    }
}

#[derive(Debug)]
pub struct Label {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub label: Option<Label>,
    pub is_checked: bool,
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
