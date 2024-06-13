use axum::Router;
use axum::routing::get;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

use crate::api::handlers::{common_handlers, todo_handlers};
use crate::api::handlers::error_handlers;

pub fn create_api_router(pool: Pool<Manager<PgConnection>>) -> Router {
    Router::new()
        .route("/", get(common_handlers::index))
        .route("/labels", get(todo_handlers::get_labels))
        .fallback(error_handlers::handle_404)
        .with_state(pool)
}