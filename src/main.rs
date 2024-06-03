use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use sea_orm::{Database, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use entity::user;

#[tokio::main]
async fn main() {
    server().await;
}

pub async fn server() {
    // * NOTE: ROUTER  _________________________________________________________________
    let app: Router = Router::new()
        .route("/api/test", get(test))
        .route("/api/user/insert", get(create_user));

    // * NOTE: SERVER _________________________________________________________________
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

async fn create_user() -> impl IntoResponse {
    // * NOTE: ════════════════════════════ DB CONNECTION ════════════════════════════
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:pepere@172.23.0.3:5432/BlogDB")
            .await
            .unwrap();

    let user_model = user::ActiveModel {
        name: Set("John".to_owned()),
        email: Set("johndoe@me.com".to_owned()),
        password: Set("secret".to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let _usr = user::Entity::insert(user_model).exec(&db).await.unwrap();

    (StatusCode::ACCEPTED, "User created").into_response()
}
