use super::repository_model::Repository;
use sqlx::{Cursor, Row};

pub async fn save_repository(
    db: &mut sqlx::pool::PoolConnection<sqlx::MySqlConnection>,
    repository: &Repository,
) -> Result<(), &'static str> {
    match sqlx::query(
        "BEGIN;
        INSERT INTO `repository_application_repositories` (`guid`, `url`, `path`)
        VALUES (?, ?, ?);",
    )
    .bind(&repository.id.value)
    .bind(&repository.url.address)
    .bind(&repository.path.value)
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err("Error saving repository to database"),
    }
}

async fn read_repository() {}
