use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use diesel::result::Error as DieselError;
use thiserror::Error;

pub fn internal_server_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    log::error!("Error found: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    DatabaseError(diesel::result::Error),

    #[error("Record not found")]
    NotFound,

    #[error("{0}")]
    Custom(StatusCode, String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Custom(status, _) => *status,
        }
    }
}

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => AppError::NotFound,
            _ => AppError::DatabaseError(error),
        }
    }
}

// This is for custom errors. If any error come from services or other internal places.
impl From<(StatusCode, String)> for AppError {
    fn from(tuple: (StatusCode, String)) -> Self {
        AppError::Custom(tuple.0, tuple.1)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = Json(serde_json::json!({ "error": self.to_string() }));
        (status_code, body).into_response()
    }
}
