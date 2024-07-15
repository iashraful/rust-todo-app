use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::todo::db_schema::labels as tbl_labels;
use crate::todo::db_schema::todos as tbl_todos;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = tbl_labels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Label {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = tbl_todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub label_id: Option<i32>,
    pub is_checked: bool,
}
