use axum::{extract::Multipart, http::StatusCode};
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

pub async fn download(mut multipart: Multipart) -> Result<StatusCode, StatusCode> {
    let mut hasher = Sha256::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let filename = field.file_name().unwrap_or("unknown");

        let path = format!("/tmp/{filename}");

        let mut file = tokio::fs::File::create(path)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut field = field;

        while let Some(chunk) = field.chunk().await.map_err(|_| StatusCode::BAD_REQUEST)? {
            file.write_all(&chunk)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            hasher.update(&chunk);
        }
    }

    let hash = hasher.finalize();
    println!("{}", hex::encode(hash));

    Ok(StatusCode::CREATED)
}
