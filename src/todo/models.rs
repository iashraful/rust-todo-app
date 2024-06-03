use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = crate::todo::schema::labels)]
pub struct Label {
    pub id: u32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::todo::schema::labels)]
pub struct NewLabel {
    pub name: String,
}


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::todo::schema::todos)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub label_id: Option<u32>,
    pub is_checked: bool,
}