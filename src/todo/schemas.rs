use diesel::prelude::*;
use serde::Deserialize;

use crate::todo::db_schema::labels as tbl_labels;
use crate::todo::db_schema::todos as tbl_todos;

#[derive(Insertable, Selectable, Queryable, Deserialize)]
#[diesel(table_name = tbl_labels)]
pub struct NewLabel {
    pub name: String,
}

#[derive(Insertable, Selectable, Queryable, Deserialize)]
#[diesel(table_name = tbl_todos)]
pub struct TodoCreate {
    pub title: String,
    pub description: String,
    pub label_id: Option<i32>,
    pub is_checked: bool,
}

#[derive(Insertable, Selectable, Queryable, Deserialize)]
#[diesel(table_name = tbl_todos)]
pub struct TodoStatusUpdate {
    pub is_checked: bool,
}

#[derive(Insertable, Selectable, Queryable, Deserialize)]
#[diesel(table_name = tbl_todos)]
pub struct TodoUpdate {
    pub title: String,
    pub description: String,
    pub label_id: Option<i32>,
}
