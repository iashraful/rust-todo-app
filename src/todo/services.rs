use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use log::debug;

use crate::api::exceptions::internal_server_error;
use crate::api::exceptions::AppError;
use crate::todo::models::{Label, NewLabel};
use crate::todo::schema::labels as tbl_labels;

pub struct TodoService {
    pub conn: Object,
}

impl TodoService {
    pub async fn get_label(&mut self, label_id: i32) -> Result<Label, AppError> {
        debug!("Fetching label from db.");
        let res = self
            .conn
            .interact(move |conn| tbl_labels::table.find(label_id).first(conn))
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(res)
    }
    pub async fn list_labels(&mut self) -> Result<Vec<Label>, AppError> {
        debug!("Fetching labels from db.");
        let res = self
            .conn
            .interact(|conn| tbl_labels::table.select(Label::as_select()).load(conn))
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(res)
    }

    pub async fn create_label(&mut self, label: NewLabel) -> Result<Label, AppError> {
        debug!("Creating label for a todo.");
        let res: Label = self
            .conn
            .interact(|conn| {
                diesel::insert_into(tbl_labels::table)
                    .values(label)
                    .returning(Label::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        debug!("Label created.");
        Ok(res)
    }

    pub async fn update_label(&mut self, pk: i32, payload: NewLabel) -> Result<Label, AppError> {
        debug!("Updating label with ID: {}.", pk);
        let res: Label = self
            .conn
            .interact(move |conn| {
                diesel::update(tbl_labels::table.filter(tbl_labels::id.eq(pk)))
                    .set(tbl_labels::name.eq(payload.name))
                    .returning(Label::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(res)
    }

    pub async fn delete_label(&mut self, pk: i32) {
        // TODO: Handle error for delete
        debug!("Deleting label with ID: {}.", pk);
        let _ = self
            .conn
            .interact(move |conn| {
                diesel::delete(tbl_labels::table.filter(tbl_labels::id.eq(pk)))
                    .execute(conn)
                    .expect("Error deleting the label.")
            })
            .await
            .map_err(internal_server_error);
    }
}
