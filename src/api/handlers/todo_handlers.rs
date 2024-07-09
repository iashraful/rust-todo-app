use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use log::{info, warn};

use crate::api::custom_codes::SUCCESS_CODE;
use crate::api::schema::BaseSchemaResponse;
use crate::core::exceptions::internal_server_error;
use crate::todo::models::{Label, NewLabel};
use crate::todo::services::TodoService;

#[debug_handler]
pub async fn get_labels(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<BaseSchemaResponse<Vec<Label>>, (StatusCode, String)> {
    info!("Retrieving the labels.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let label_data: Vec<Label> = todo_srv.list_labels().await?;
    info!("DB Query finished.");
    let resp: BaseSchemaResponse<Vec<Label>> = BaseSchemaResponse {
        data: label_data,
        code: SUCCESS_CODE.to_string(),
        msg: String::from("Request process successfully."),
    };
    Ok(resp)
}

#[debug_handler]
pub async fn create_label(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(payload): Json<NewLabel>,
) -> Result<Json<Label>, (StatusCode, String)> {
    info!("Creating new label.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let res = todo_srv.create_label(payload).await;
    if res.is_ok() {
        info!("New label has created.");
    } else {
        warn!("Failed to create label.")
    }
    res
}

#[debug_handler]
pub async fn update_label(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(pk): Path<i32>,
    Json(payload): Json<NewLabel>,
) -> Result<Json<Label>, (StatusCode, String)> {
    info!("Updating a label.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let res = todo_srv.update_label(pk, payload).await;
    if res.is_ok() {
        info!("Label has updated.");
    } else {
        warn!("Failed to update label.")
    }
    res
}

// #[debug_handler]
// pub async fn delete_label(
//     State(pool): State<deadpool_diesel::postgres::Pool>,
//     Path(pk): Path<i32>,
// ) -> (StatusCode, String) {
//     info!("Deleting a label.");
//     let conn = pool.get().await.map_err(internal_server_error)?;
//     let mut todo_srv = TodoService { conn };
//     let res = todo_srv.delete_label(pk).await;
//     if res.is_ok() {
//         info!("Label has deleted.");
//     } else {
//         warn!("Failed to delete label.")
//     }
//     res
// }
