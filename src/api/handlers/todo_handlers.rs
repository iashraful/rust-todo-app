use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use log::info;

use crate::api::custom_codes::{CREATED_CODE, DELETED_CODE, SUCCESS_CODE, UPDATED_CODE};
use crate::api::schema::{BaseAPIResponse, DeleteAPIResponse};
use crate::core::exceptions::internal_server_error;
use crate::todo::models::{Label, NewLabel};
use crate::todo::services::TodoService;

#[debug_handler]
pub async fn get_labels(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<BaseAPIResponse<Vec<Label>>, (StatusCode, String)> {
    info!("Retrieving the labels.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let label_data: Vec<Label> = todo_srv.list_labels().await?;
    info!("DB Query finished.");
    let resp: BaseAPIResponse<Vec<Label>> = BaseAPIResponse {
        data: label_data,
        code: SUCCESS_CODE.to_string(),
        msg: String::from("Request process successfully."),
    };
    Ok(resp)
}

#[debug_handler]
pub async fn retrieve_label(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(pk): Path<i32>,
) -> Result<BaseAPIResponse<Label>, (StatusCode, String)> {
    info!("Retrieving the label with ID: {}.", pk);
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let label_data: Label = todo_srv.get_label(pk).await?;
    info!("DB Query finished.");
    let resp: BaseAPIResponse<Label> = BaseAPIResponse {
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
) -> Result<BaseAPIResponse<Label>, (StatusCode, String)> {
    info!("Creating new label.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let label_data = todo_srv.create_label(payload).await?;
    info!("New label has created.");
    let resp: BaseAPIResponse<Label> = BaseAPIResponse {
        data: label_data,
        code: CREATED_CODE.to_string(),
        msg: String::from("Request process successfully."),
    };
    Ok(resp)
}

#[debug_handler]
pub async fn update_label(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(pk): Path<i32>,
    Json(payload): Json<NewLabel>,
) -> Result<BaseAPIResponse<Label>, (StatusCode, String)> {
    info!("Updating a label.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let label_data = todo_srv.update_label(pk, payload).await?;
    info!("Label has updated.");
    let resp: BaseAPIResponse<Label> = BaseAPIResponse {
        data: label_data,
        code: UPDATED_CODE.to_string(),
        msg: String::from("Request process successfully."),
    };
    Ok(resp)
}

#[debug_handler]
pub async fn delete_label(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(pk): Path<i32>,
) -> Result<DeleteAPIResponse, (StatusCode, String)> {
    info!("Deleting a label.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    todo_srv.delete_label(pk).await;
    info!("Label has deleted.");
    let resp: DeleteAPIResponse = DeleteAPIResponse {
        code: DELETED_CODE.to_string(),
        msg: String::from("Request process successfully."),
    };
    Ok(resp)
}
