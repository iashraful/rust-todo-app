use diesel::prelude::*;

use crate::todo::schema::labels as tbl_labels;
use crate::todo::schema::todos as tbl_todos;

#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = tbl_labels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Label {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = tbl_labels)]
pub struct NewLabel {
    pub name: String,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = tbl_todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub label_id: Option<i32>,
    pub is_checked: bool,
}


#[derive(Insertable)]
#[diesel(table_name = tbl_todos)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    pub label_id: Option<i32>,
    pub is_checked: bool,
}