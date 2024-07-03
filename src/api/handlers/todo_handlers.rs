use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use log::{info, warn};

use crate::core::exceptions::internal_server_error;
use crate::todo::models::{Label, NewLabel};
use crate::todo::services::TodoService;

#[debug_handler]
pub async fn get_labels(
    State(pool): State<deadpool_diesel::postgres::Pool>
) -> Result<Json<Vec<Label>>, (StatusCode, String)> {
    info!("Retrieving the labels.");
    let conn = pool.get().await.map_err(internal_server_error)?;
    let mut todo_srv = TodoService { conn };
    let res = todo_srv.list_labels().await;
    info!("DB Query finished.");
    res
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

