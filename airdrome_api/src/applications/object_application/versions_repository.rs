use super::id_model::Id;
use super::versions_factory;
use super::versions_model::{Version, Versions};
use chrono::NaiveDateTime;
use sqlx::{Executor, Row};

pub async fn save_versions(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    object_id: u64,
    versions: &Versions,
) -> Result<(), &'static str> {
    let mut db_connection = match db_pool.acquire().await {
        Ok(connection) => connection,
        Err(_) => panic!("Unable to open db connection"),
    };

    for version in &versions.all {
        match db_connection
            .execute(sqlx::query!(
                "INSERT INTO `object_application_versions`
                (`id`, `number`, `created_timestamp`, `commit`, `zip_hash`, `object_id`)
                VALUES (?, ?, ?, ?, ?, ?);",
                &version.id.uuid(),
                version.number.value,
                version.created_timestamp.value,
                version.commit.value,
                version.zip_hash.value,
                object_id,
            ))
            .await
        {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                return Err("Error saving object version");
            }
        };
    }

    Ok(())
}

pub async fn read_versions(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    object_id: u64,
) -> Result<Versions, &'static str> {
    let mut db_connection = match db_pool.acquire().await {
        Ok(connection) => connection,
        Err(_) => panic!("Unable to open db connection"),
    };

    let mut rows = match db_connection
        .fetch_all(sqlx::query!(
            "SELECT `id`, `number`, `created_timestamp`, `commit`, `zip_hash`
            FROM object_application_versions AS version
            WHERE object_id = ?;",
            object_id
        ))
        .await
    {
        Ok(r) => r,
        Err(_) => return Err("Error reading object versions"),
    };

    let mut versions: Vec<Version> = Vec::new();

    for row in rows {
        match versions_factory::restore_version(
            row.get("id"),
            row.get("number"),
            row.get("commit"),
            row.get("zip_hash"),
            // &NaiveDateTime::parse_from_str(row.get("created_timestamp"), "%Y-%m-%dT%H:%M:%S%z")
            // .expect("")
            // .to_string(),
            &row.get::<NaiveDateTime, &str>("created_timestamp")
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string(),
        ) {
            Ok(version) => versions.push(version),
            Err(e) => {
                println!("{}", e);
                return Err("Error reading object version");
            }
        }
    }

    versions_factory::create_versions(versions)
}
