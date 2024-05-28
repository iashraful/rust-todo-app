use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::labels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Label {
    pub id: u32,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::3schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub label: Option<Label>,
    pub is_checked: bool,
}