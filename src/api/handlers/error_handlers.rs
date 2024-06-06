use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}