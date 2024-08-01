use axum::{extract::Path, http::StatusCode, Extension, Json};

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter,
    QuerySelect, Set,
};
use serde_json::Value;
use uuid::Uuid;

use crate::{models::post_models::CreatePostModel, utils::api_error::APIError};

// ______________________________________________________________________
pub async fn create_post_post(
    Extension(db): Extension<DatabaseConnection>,
    Extension(identity): Extension<entity::user::Model>,
    Json(post_data): Json<CreatePostModel>,
) -> Result<(), APIError> {
    let post_entity = entity::post::ActiveModel {
        title: Set(post_data.title),
        text: Set(post_data.text),
        image: Set(post_data.image),
        created_at: Set(Utc::now().naive_local()),
        user_id: Set(identity.id),
        uuid: Set(Uuid::new_v4()),
        ..Default::default()
    };

    post_entity.insert(&db).await.map_err(|_| APIError {
        message: "Filed to insert".to_owned(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    })?;

    Ok(())
}

// ______________________________________________________________________
pub async fn get_post_get(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Value>, APIError> {
    let post = entity::post::Entity::find()
        .filter(entity::post::Column::Uuid.eq(uuid))
        // .find_also_related(entity::user::Entity)
        .column_as(entity::user::Column::Name, "author")
        .column_as(entity::user::Column::Uuid, "author uuid")
        .join(
            JoinType::LeftJoin,
            entity::post::Entity::belongs_to(entity::user::Entity)
                .from(entity::post::Column::UserId)
                .to(entity::user::Column::Id)
                .into(),
        )
        .into_json()
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Not Found".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?;

    Ok(Json(post))
}
