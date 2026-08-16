use axum::http::header;
use axum::response::IntoResponse;
use tokio::fs;

pub async fn send_image() -> impl IntoResponse {
    let bytes = fs::read("/home/tuxzilla/Downloads/image.webp")
        .await
        .unwrap();
    ([(header::CONTENT_TYPE, "image/webp")], bytes)
}
