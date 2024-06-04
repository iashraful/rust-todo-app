use diesel::{PgConnection, RunQueryDsl};
use log::info;

use crate::todo::models::{Label, NewLabel};
use crate::todo::schema::labels as tbl_labels;

pub struct TodoService {
    pub conn: PgConnection,
}

impl TodoService {
    pub fn create_label(&mut self, label: NewLabel) {
        info!("Creating label for a todo.");
        diesel::insert_into(tbl_labels::table)
            .values(&label)
            .execute(&mut self.conn)
            .expect("Error creating label.");
    }
}