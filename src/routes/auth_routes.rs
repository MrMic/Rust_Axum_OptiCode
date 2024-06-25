use axum::{http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::auth_handlers;

pub fn auth_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any);

    let router = Router::new()
        .route("/api/user/register", post(auth_handlers::create_user_post))
        .route("/api/user/login", post(auth_handlers::login_user_post))
        .layer(cors);

    router
}
