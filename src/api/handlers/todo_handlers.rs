use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use log::info;

use crate::core::exceptions::internal_server_error;
use crate::todo::models::Label;
use crate::todo::services::TodoService;

pub async fn get_labels(
    State(pool): State<deadpool_diesel::postgres::Pool>
) -> Result<Json<Vec<Label>>, (StatusCode, String)> {
    info!("Getting the labels from database.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let res = todo_srv.list_labels().await;
    info!("DB Query finished.");
    res
}

