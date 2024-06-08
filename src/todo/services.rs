use diesel::{PgConnection, QueryResult, RunQueryDsl};
use diesel::prelude::*;
use log::{debug, info};

use crate::todo::models::{Label, NewLabel};
use crate::todo::schema::labels as tbl_labels;

pub struct TodoService {
    pub conn: PgConnection,
}

impl TodoService {
    pub async fn create_label(&mut self, label: NewLabel) -> QueryResult<Label> {
        info!("Creating label for a todo.");
        diesel::insert_into(tbl_labels::table)
            .values(&label)
            .returning(Label::as_returning())
            .get_result(&mut self.conn)
    }

    pub async fn list_labels(&mut self) -> QueryResult<Vec<Label>> {
        debug!("Fetching labels from db.");
        tbl_labels::table.select(Label::as_select()).load(&mut self.conn)
    }
}