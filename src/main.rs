use axum::{middleware, Extension, Router};
use migration::sea_orm::Database;
use tower_http::services::ServeDir;

mod handlers;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    server().await;
}

pub async fn server() {
    // * INFO: DB Connection ___________________________________________________________
    let conn_str = (*utils::constants::DATABASE_URL).clone();
    let db = Database::connect(conn_str)
        .await
        .expect("Failed to connect to DB");

    // * INFO: ROUTER  _________________________________________________________________
    let app: Router = Router::new()
        .merge(routes::user_routes::user_routes())
        .route_layer(middleware::from_fn(utils::guards::guard))
        .merge(routes::auth_routes::auth_routes())
        .merge(routes::home_routes::home_routes())
        .layer(Extension(db))
        .nest_service("/", ServeDir::new("public"));

    // * INFO: SERVER _________________________________________________________________
    // * INFO: run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
