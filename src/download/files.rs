use axum::{extract::Multipart, http::StatusCode};
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

use crate::State;
use crate::download::db::insert_file;

// i think my next best step here is to filter out say..
// someone uploading two files at once!
// cause i think this just merges two files into one blob.
// which.. isn't helpful.
// also need to figure out what happens when two people upload at the same time

pub async fn download(
    axum::extract::State(state): axum::extract::State<State>,
    mut multipart: Multipart,
) -> Result<StatusCode, StatusCode> {
    //                     ^ replace this with an actual error type
    let mut hasher = Sha256::new();
    let mut file_size_bytes: u64 = 0;
    let mut detected_mimetype: Option<String> = None;
    let mut multipart_mimetype: Option<String> = None;

    let temp = tempfile::Builder::new()
        .prefix(".temp-spool-")
        .tempfile_in("/home/spool")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // also we need an actual logging system .. not this

    let std_file = temp
        .as_file()
        .try_clone()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut file = tokio::fs::File::from_std(std_file);

    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        if multipart_mimetype.is_none() {
            multipart_mimetype = field.content_type().map(str::to_owned);
        }

        while let Some(chunk) = field.chunk().await.map_err(|_| StatusCode::BAD_REQUEST)? {
            if detected_mimetype.is_none() {
                detected_mimetype = infer::get(&chunk).map(|kind| kind.mime_type().to_owned());
            }

            file.write_all(&chunk)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            hasher.update(&chunk);

            file_size_bytes = file_size_bytes
                .checked_add(chunk.len() as u64)
                .ok_or(StatusCode::PAYLOAD_TOO_LARGE)?;
        }
    }

    file.flush()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let hash_hex = hex::encode(hasher.finalize());
    let final_path = format!("/home/spool/{hash_hex}");

    let mimetype = detected_mimetype
        .or(multipart_mimetype)
        .unwrap_or_else(|| "application/octet-stream".to_owned());

    let file_size_bytes =
        i64::try_from(file_size_bytes).map_err(|_| StatusCode::PAYLOAD_TOO_LARGE)?;

    let result = insert_file(&state.db, &hash_hex, &mimetype, file_size_bytes).await;

    if let Err(e) = result {
        match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                drop(temp);
                return Err(StatusCode::OK);
            }
            _ => {}
        };
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    temp.persist(&final_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!(
        "DEBUG: {} ({}, {} bytes)",
        final_path, mimetype, file_size_bytes
    );

    Ok(StatusCode::CREATED)
}
