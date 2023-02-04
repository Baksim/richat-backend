mod index;
pub mod posts;
use axum::{http::Method, routing::get, Router};
use index::hello;
use posts::posts;
use tower_http::cors::{Any, CorsLayer};

use crate::AppState;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(hello))
        .route("/posts", get(posts))
        .layer(cors)
        .with_state(state)
}
