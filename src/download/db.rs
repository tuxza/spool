use sqlx::AnyPool;
use sqlx::query;

pub async fn insert_file(
    db: &AnyPool,
    hash: &str,
    original_filename: &str,
) -> Result<(), sqlx::Error> {
    query(
        r#"
        INSERT INTO files (
            hash_filename,
            original_filename,
            reference_count
        )
        VALUES (?, ?, 1)
        "#,
    )
    .bind(hash)
    .bind(original_filename)
    .execute(db)
    .await?;

    Ok(())
}
