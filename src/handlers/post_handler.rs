use axum::{extract::Path, http::StatusCode, Extension, Json};

use axum_extra::extract::Multipart;
use chrono::Utc;

use std::io::BufWriter;

use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use image::ImageReader;

use fast_image_resize::images::Image;
use fast_image_resize::{IntoImageView, Resizer};

use migration::sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect, Set,
};
use migration::{Condition, JoinType};

use serde_json::Value;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{models::post_models::CreatePostModel, utils::api_error::APIError};

// ______________________________________________________________________
pub async fn upload_image_post(
    Extension(db): Extension<DatabaseConnection>,
    Extension(identity): Extension<entity::user::Model>,
    Path(uuid): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<(), APIError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        if field_name == "image" {
            let mut post = entity::post::Entity::find()
                .filter(
                    Condition::all()
                        .add(entity::post::Column::Uuid.eq(uuid))
                        .add(entity::post::Column::UserId.eq(identity.id)),
                )
                .one(&db)
                .await
                .unwrap()
                .unwrap()
                .into_active_model();

            let img_name: i64 = Utc::now().timestamp();
            let data = field.bytes().await.unwrap();

            // * INFO: ____________________________ RESIZE IMAGE _________________________
            // Read source image from file
            let src_image = ImageReader::new(std::io::Cursor::new(data))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();

            // Create container for data of destination image
            let dst_width = 480;
            let dst_height = 360;
            let mut dst_image = Image::new(dst_width, dst_height, src_image.pixel_type().unwrap());

            // Create Resizer instance and resize source image
            // into buffer of destination image
            let mut resizer = Resizer::new();
            resizer.resize(&src_image, &mut dst_image, None).unwrap();

            // Write destination image as PNG-file
            let mut result_buf = BufWriter::new(Vec::new());
            PngEncoder::new(&mut result_buf)
                .write_image(
                    dst_image.buffer(),
                    dst_width,
                    dst_height,
                    src_image.color().into(),
                )
                .unwrap();
            // ______________________________________________________________________
            let image_bytes = result_buf.into_inner().unwrap();

            let mut file = File::create(format!("./public/uploads/{}.png", img_name))
                .await
                .unwrap();

            file.write_all(&image_bytes).await.unwrap();

            post.image = Set(format!("./public/uploads/{}.png", img_name));
            post.update(&db).await.unwrap();
            println!("/uploads/{}.png", img_name);
        } else {
            let data = field.text().await.unwrap();
            println!("field: {}  value: {}", field_name, data);
        }
    }

    Ok(())
}

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
