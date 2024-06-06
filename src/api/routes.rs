use axum::Router;
use axum::routing::get;

use crate::api::handlers::common_handlers;
use crate::api::handlers::error_handlers;

pub fn create_api_router() -> Router {
    Router::new()
        .route("/", get(common_handlers::index))
        .fallback(error_handlers::handle_404)
}