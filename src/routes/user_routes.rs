use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::{
    post_handler::create_post_post,
    user_handler::{all_user_get, delete_user_delete, update_user_put},
};

pub fn user_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::DELETE, Method::POST, Method::PUT])
        .allow_origin(Any);

    let router = Router::new()
        .route("/api/user/:uuid/update", put(update_user_put))
        .route("/api/user/:uuid/delete", delete(delete_user_delete))
        .route("/api/user/all", get(all_user_get))
        .route("/api/user/post", post(create_post_post))
        .layer(cors);

    router
}
