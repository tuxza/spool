use axum::{Router, routing::get};

async fn upload() {
    // we need to accept a file here
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "spool server is running!" }))
        .route("/upload", post(upload));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
