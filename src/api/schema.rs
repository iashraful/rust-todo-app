use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct BaseSchemaResponse<T> {
    pub data: T,
    pub msg: String,
    pub code: String,
}

impl<T> IntoResponse for BaseSchemaResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "data": self.data,
            "msg": self.msg,
            "code": self.code,
        });
        (StatusCode::OK, Json(body)).into_response()
    }
}
