use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::utils::api_error::APIError;

use super::jwt::decode_jwt;

// * INFO: This file contains the guards for the routes in the API.
// The guards are used to check if the user is authorized to access the route.
pub async fn guard(mut req: Request<Body>, next: Next) -> Result<Response, APIError> {
    let token = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(APIError {
            message: "No auth toke found".to_owned(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(40),
        })?
        .token()
        .to_owned();

    let claim = decode_jwt(token)
        .map_err(|_| APIError {
            message: "Unauthorized".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(41),
        })?
        .claims;

    let db = req
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(APIError {
            message: "Could not connect to database".to_owned(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    let identity = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(claim.email.to_lowercase()))
        .one(db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Unauthorized".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(41),
        })?;

    req.extensions_mut().insert(identity);

    Ok(next.run(req).await)
}
