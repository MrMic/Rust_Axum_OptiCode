use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use sea_orm::ActiveModelTrait;
use sea_orm::{Database, DatabaseConnection, Set};
use uuid::Uuid;

use entity::user;

use crate::models::user_models::CreateUserModel;

mod models;

#[tokio::main]
async fn main() {
    server().await;
}

pub async fn server() {
    // * INFO: ROUTER  _________________________________________________________________
    let app: Router = Router::new()
        .route("/api/test", get(test))
        .route("/api/user/insert", post(create_user_post));

    // * INFO: SERVER _________________________________________________________________
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn test() -> impl IntoResponse {
    println!("test API");

    (StatusCode::ACCEPTED, "Hi there")
}

async fn create_user_post(Json(user_data): Json<CreateUserModel>) -> impl IntoResponse {
    // * INFO: ════════════════════════════ DB CONNECTION ════════════════════════════
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:pepere@172.23.0.2:5432/BlogDB")
            .await
            .unwrap();

    let user_model = user::ActiveModel {
        name: Set(user_data.name.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(user_data.password.to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    user_model.insert(&db).await.unwrap();

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, "User created").into_response()
}
