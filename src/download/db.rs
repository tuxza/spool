use sqlx::AnyPool;
use sqlx::query;

pub async fn insert_file(
    db: &AnyPool,
    hash: &str,
    mimetype: &str,
    file_size_bytes: i64,
) -> Result<(), sqlx::Error> {
    query(
        r#"
        INSERT INTO files (
            hash_filename,
            mimetype,
            file_size_bytes
        )
        VALUES (?, ?, ?)
        "#,
    )
    .bind(hash)
    .bind(mimetype)
    .bind(file_size_bytes)
    .execute(db)
    .await?;

    Ok(())
}
