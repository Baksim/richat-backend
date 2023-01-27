mod index;
use axum::{Router, routing::get};
use index::hello;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello))
}
