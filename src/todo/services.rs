use axum::http::StatusCode;
use axum::response::Json;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use log::debug;

use crate::core::exceptions::internal_server_error;
use crate::todo::models::{Label, NewLabel};
use crate::todo::schema::labels as tbl_labels;

pub struct TodoService {
    pub conn: Object,
}

impl TodoService {
    pub async fn list_labels(&mut self) -> Result<Vec<Label>, (StatusCode, String)> {
        debug!("Fetching labels from db.");
        let res = self
            .conn
            .interact(|conn| tbl_labels::table.select(Label::as_select()).load(conn))
            .await
            .map_err(internal_server_error)?
            .map_err(internal_server_error)?;
        Ok(res)
    }

    pub async fn create_label(
        &mut self,
        label: NewLabel,
    ) -> Result<Json<Label>, (StatusCode, String)> {
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
        Ok(Json(res))
    }

    pub async fn update_label(
        &mut self,
        pk: i32,
        payload: NewLabel,
    ) -> Result<Json<Label>, (StatusCode, String)> {
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
        Ok(Json(res))
    }

    // pub async fn delete_label(
    //     &mut self,
    //     pk: i32,
    // ) -> (StatusCode, String) {
    //     debug!("Deleting label with ID: {}.", pk);
    //     self.conn
    //         .interact(move |conn| {
    //             diesel::delete(tbl_labels::table.filter(tbl_labels::id.eq(pk)))
    //                 .execute(conn)
    //                 .expect("Error deleting the label.")
    //         })
    //         .await
    //         .map_err(internal_server_error)?
    //         .map_err(internal_server_error)?;
    // }
}
