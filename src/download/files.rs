use axum::{extract::Multipart, http::StatusCode};
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

use crate::State;
use crate::download::db::insert_file;

pub async fn download(
    axum::extract::State(state): axum::extract::State<State>,
    mut multipart: Multipart,
) -> Result<StatusCode, StatusCode> {
    let mut hasher = Sha256::new();

    let temp = tempfile::Builder::new()
        .prefix(".temp-spool-")
        .tempfile_in("/home/spool")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let std_file = temp
        .as_file()
        .try_clone()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut file = tokio::fs::File::from_std(std_file);

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let mut field = field;

        while let Some(chunk) = field.chunk().await.map_err(|_| StatusCode::BAD_REQUEST)? {
            file.write_all(&chunk)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            hasher.update(&chunk);
        }
    }

    file.flush()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let hash_hex = hex::encode(hasher.finalize());
    let final_path = format!("/home/spool/{hash_hex}");

    insert_file(&state.db, &hash_hex, &final_path)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    drop(file);

    temp.persist(&final_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("DEBUG: {}", final_path);

    Ok(StatusCode::CREATED)
}
