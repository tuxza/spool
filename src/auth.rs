use crate::entities::users::{self, Column::Username};

use axum::{Json, extract::State, http::StatusCode};
use sea_orm::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    let login = users::Entity::find()
        .filter(Username.eq(payload.username))
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(user) = login
        && user.psd == payload.password
    {
        return Ok(StatusCode::OK);
    }
    Err((
        StatusCode::UNAUTHORIZED,
        "Invalid username or password".to_string(),
    ))
}
