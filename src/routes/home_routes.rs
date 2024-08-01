use axum::{http::Method, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::post_handler;

pub fn home_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    let router = Router::new()
        .route("/api/post/:uuid", get(post_handler::get_post_get))
        .layer(cors);

    router
}
