use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use log::info;

use crate::api::exceptions::{internal_server_error, AppError};
use crate::api::response_codes::{CREATED_CODE, DELETED_CODE, SUCCESS_CODE, UPDATED_CODE};
use crate::api::schema::{BaseAPIResponse, DeleteAPIResponse};
use crate::todo::models::{Label, Todo};
use crate::todo::schemas::{NewLabel, TodoCreate};
use crate::todo::services::TodoService;

#[debug_handler]
pub async fn get_labels(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<BaseAPIResponse<Vec<Label>>, AppError> {
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
) -> Result<BaseAPIResponse<Label>, AppError> {
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
) -> Result<BaseAPIResponse<Label>, AppError> {
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
) -> Result<BaseAPIResponse<Label>, AppError> {
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
) -> Result<DeleteAPIResponse, AppError> {
    info!("Deleting a label.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    todo_srv.delete_label(pk).await?;
    info!("Label has deleted.");
    let resp: DeleteAPIResponse = DeleteAPIResponse {
        code: DELETED_CODE.to_string(),
        msg: String::from("Request process successfully."),
    };
    Ok(resp)
}

#[debug_handler]
pub async fn get_todos(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<BaseAPIResponse<Vec<Todo>>, AppError> {
    info!("Retrieving the labels .");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let data: Vec<Todo> = todo_srv.list_todos().await?;
    info!("DB Query finished.");
    let resp: BaseAPIResponse<Vec<Todo>> = BaseAPIResponse {
        data: data,
        code: SUCCESS_CODE.to_string(),
        msg: String::from("Request process successfully."),
    };
    Ok(resp)
}

#[debug_handler]
pub async fn create_todo(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(payload): Json<TodoCreate>,
) -> Result<BaseAPIResponse<Todo>, AppError> {
    info!("Creating new todo.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let data = todo_srv.create_todo(payload).await?;
    info!("New todo has created.");
    let resp: BaseAPIResponse<Todo> = BaseAPIResponse {
        data: data,
        code: CREATED_CODE.to_string(),
        msg: String::from("Request process successfully."),
    };
    Ok(resp)
}
