use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub name: String,
    pub email: String,
    pub password: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub password: String,
}
