use axum::http::StatusCode;
use axum::response::Json;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use log::{debug, info};

use crate::core::exceptions::internal_server_error;
use crate::todo::models::{Label, NewLabel};
use crate::todo::schema::labels as tbl_labels;

pub struct TodoService {
    pub conn: Object,
}

impl TodoService {
    pub async fn create_label(
        &mut self, label: NewLabel,
    ) -> Result<Json<Label>, (StatusCode, String)> {
        info!("Creating label for a todo.");
        let res: Label = self.conn
            .interact(|conn| {
                diesel::insert_into(tbl_labels::table)
                    .values(label)
                    .returning(Label::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(Json(res))
    }

    pub async fn list_labels(&mut self) -> Result<Json<Vec<Label>>, (StatusCode, String)> {
        debug!("Fetching labels from db.");
        let res = self.conn
            .interact(|conn| tbl_labels::table.select(Label::as_select()).load(conn))
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(Json(res))
    }
}