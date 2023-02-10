use super::id_factory;
use super::id_model::Id;
use super::object_factory;
use super::object_model::Object;
use super::versions_repository;
use sqlx::{Executor, Row};
use std::convert::TryInto;

pub async fn save_object(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    object: &Object,
) -> Result<(), &'static str> {
    let mut object_targets_values: Vec<String> = Vec::new();
    let mut object_languages_values: Vec<String> = Vec::new();

    for target in object.targets.value.iter() {
        object_targets_values.push(format!(
            "(@new_object_id, (
                SELECT `uuid` FROM `object_application_targets` WHERE `name` = '{}'))",
            target
        ));
    }

    for language in object.languages.value.iter() {
        object_languages_values.push(format!(
            "(@new_object_id, (
                SELECT `uuid` FROM `object_application_languages` WHERE `name` = '{}'))",
            language
        ));
    }

    // match db
    //     .execute("START TRANSACTION")
    //     .await
    //     .and(
    //         db.execute(sqlx::query!(
    //             "INSERT INTO `object_application_objects` (`guid`, `name`, `description`) VALUES (?, ?, ?);",
    //             object.id.value,
    //             object.name.value,
    //             object.description.value,
    //         ))
    //         .await,
    //     )
    //     .and(
    //         db.execute(
    //             format!(
    //                 "
    //     INSERT INTO `object_application_object_targets` (object_id, target_id)
    //     VALUES {};",
    //                 object_targets_values.join(", ")
    //             )
    //             .as_str(),
    //         )
    //         .await,
    //     )
    //     .and(
    //         db.execute(
    //             format!(
    //                 "
    //     INSERT INTO `object_application_object_targets` (object_id, language_id)
    //     VALUES {};",
    //                 object_languages_values.join(", ")
    //             )
    //             .as_str(),
    //         )
    //         .await,
    //     )
    //     .and(db.execute("COMMIT;").await)
    //     .or(db.execute("ROLLBACK;").await)

    let mut db_connection = match db_pool.acquire().await {
        Ok(connection) => connection,
        Err(_) => panic!("Unable to open db connection"),
    };
    let uuid = object.id.uuid();

    match db_connection
        .execute(sqlx::query!(
            "INSERT INTO `object_application_objects`
            (`uuid`, `name`, `description`)
            VALUES (?, ?, ?);",
            uuid,
            object.name.value,
            object.description.value,
        ))
        .await
    {
        Ok(r) => r.last_insert_rowid(),
        Err(e) => panic!("{:?}", e),
    };

    println!("object_id: {:?}", &uuid);
    versions_repository::save_versions(db_pool, &uuid, &object.versions)
        .await
        .expect("Error saving object versions");

    Ok(())
}

pub async fn read_object(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    id: &Id,
) -> Result<Object, &'static str> {
    let mut db_connection = match db_pool.acquire().await {
        Ok(connection) => connection,
        Err(_) => panic!("Unable to open db connection"),
    };

    // TODO make this a compiled query (see SQLX 0.6 bug)
    let mut rows = match db_connection
        .fetch_all(
            sqlx::query(
                "SELECT object.uuid AS id, object.name AS name, object.description AS description,
            GROUP_CONCAT(REPLACE(DISTINCT(target.name), '', ''), ',') AS targets,
            GROUP_CONCAT(REPLACE(DISTINCT(language.name), '', ''), ',') AS languages
            FROM object_application_objects AS object
            LEFT JOIN object_application_object_languages AS object_languages
            ON object.id = object_languages.object_id
            LEFT JOIN object_application_languages AS language
            ON language.id = object_languages.language_id
            LEFT JOIN object_application_object_targets AS object_targets
            ON object.id = object_targets.object_id
            LEFT JOIN object_application_targets AS target
            ON target.id = object_targets.target_id
            WHERE object.uuid = ?
            GROUP BY object.id;",
            )
            .bind(&id.uuid()),
        )
        .await
    {
        Ok(r) => r,
        Err(_) => return Err("Error searching the database"),
    };

    let mut objects: Vec<Object> = Vec::new();

    for row in rows {
        // let row = match row {
        //     Some(row) => row,
        //     None => break,
        // };
        let id = id_factory::create_id(Some(row.get("id")))?;

        let versions = versions_repository::read_versions(db_pool, &id)
            .await
            .expect("Error restoring object versions");

        match object_factory::restore_object(
            row.get("id"),
            row.get("name"),
            row.get("description"),
            row.get::<Option<&str>, &str>("languages")
                .unwrap_or("")
                .split_terminator(",")
                .collect::<Vec<&str>>(),
            row.get::<Option<&str>, &str>("targets")
                .unwrap_or("")
                .split_terminator(",")
                .collect::<Vec<&str>>(),
            versions,
        ) {
            Ok(object) => objects.push(object),
            Err(_) => return Err("Error reading object from database"),
        }
    }

    match objects.len() {
        0 => Err("No object found with that Id"),
        1 => Ok(objects[0].clone()),
        _ => Err("More than one object found with same Id"),
    }
}

pub async fn read_objects(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    name: Option<&str>,
    targets: Option<Vec<&str>>,
    languages: Option<Vec<&str>>,
    keywords: Option<&str>,
) -> Result<Vec<Object>, &'static str> {
    let name = name.unwrap_or("");

    let mut targets = match targets {
        Some(targets) => targets.join("','"),
        None => String::new(),
    };
    if !targets.is_empty() {
        targets.insert_str(0, "'");
        targets.push('\'');
    }

    let mut languages = match languages {
        Some(languages) => languages.join("','"),
        None => String::new(),
    };
    if !languages.is_empty() {
        languages.insert_str(0, "'");
        languages.push('\'');
    }

    let keywords = keywords.unwrap_or("");

    let mut db_connection = match db_pool.acquire().await {
        Ok(connection) => connection,
        Err(_) => panic!("Unable to open db connection"),
    };

    // TODO make this a compiled query (see SQLX 0.6 bug)
    let mut rows = match db_connection
        .fetch_all(
            sqlx::query(
                r#"SELECT object.uuid AS id, object.name AS name, object.description AS description,
            GROUP_CONCAT(DISTINCT target.name) AS targets,
            GROUP_CONCAT(DISTINCT language.name) AS languages
            FROM object_application_objects AS object
            LEFT JOIN object_application_object_targets AS object_targets
            ON object.id = object_targets.object_id
            LEFT JOIN object_application_targets AS target
            ON target.id = object_targets.target_id
            LEFT JOIN object_application_object_languages AS object_languages
            ON object.id = object_languages.object_id
            LEFT JOIN object_application_languages AS language
            ON language.id = object_languages.language_id
            WHERE (object.name LIKE ? OR target.name IN (?) OR language.name IN (?))
            GROUP BY object.id;"#,
            )
            .bind(format!("%{}%", name))
            .bind(targets)
            .bind(languages),
        )
        .await
    {
        Ok(r) => r,
        Err(e) => {
            println!("{:?}", e);
            return Err("Error reading objects");
        }
    };

    let mut objects: Vec<Object> = Vec::new();

    for row in rows {
        // TODO apparently sqlite will return something no matter what (empty row),
        // so we have to account for this.
        if row.get::<&str, &str>("id") == "" {
            break;
        }

        let id = id_factory::create_id(Some(row.get("id")))?;
        let versions = versions_repository::read_versions(db_pool, &id)
            .await
            .expect("Error restoring object versions");

        match object_factory::restore_object(
            row.get("id"),
            row.get("name"),
            row.get("description"),
            // row.get::<Option<&str>, &str>("languages")
            //     .unwrap_or("")
            //     .split_terminator(",")
            //     .collect::<Vec<&str>>(),
            Vec::new(),
            // row.get::<Option<&str>, &str>("targets")
            //     .unwrap_or("")
            //     .split_terminator(",")
            //     .collect::<Vec<&str>>(),
            Vec::new(),
            versions,
        ) {
            Ok(object) => objects.push(object),
            Err(e) => panic!("{:?}", e),
        }
    }

    Ok(objects)
}
