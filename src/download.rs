use axum::{body::Body, http::StatusCode};

pub async fn download() -> Result<Body, StatusCode> {
    Ok(Body::from("stupid http code here"))
}
