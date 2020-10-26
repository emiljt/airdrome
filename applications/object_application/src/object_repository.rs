use super::object_factory;
use super::object_model::Object;
use sqlx::{Executor, Row};

async fn save_object(
    mut db: sqlx::pool::PoolConnection<sqlx::MySql>,
    object: Object,
) -> Result<(), &'static str> {
    let mut object_targets_values: Vec<String> = Vec::new();
    let mut object_languages_values: Vec<String> = Vec::new();

    for target in object.targets.value.iter() {
        object_targets_values.push(format!(
            "(@new_object_id, (SELECT `id` FROM `object_application_targets` WHERE `name` = '{}'))",
            target
        ));
    }

    for language in object.languages.value.iter() {
        object_targets_values.push(format!(
            "(@new_object_id, (SELECT `id` FROM `object_application_languages` WHERE `name` = '{}'))",
            language
        ));
    }

    match db
        .execute("START TRANSACTION")
        .await
        .and(
            db.execute(sqlx::query!(
                "INSERT INTO `object_application_objects` (`guid`, `name`, `description`)
        VALUES (?, ?, ?);",
                object.id.value,
                object.name.value,
                object.description.value,
            ))
            .await,
        )
        .and(
            db.execute(
                format!(
                    "
        INSERT INTO `object_application_object_targets` (object_id, target_id)
        VALUES {};",
                    object_targets_values.join(", ")
                )
                .as_str(),
            )
            .await,
        )
        .and(
            db.execute(
                format!(
                    "
        INSERT INTO `object_application_object_targets` (object_id, language_id)
        VALUES {};",
                    object_languages_values.join(", ")
                )
                .as_str(),
            )
            .await,
        )
        .and(db.execute("COMMIT;").await)
        .or(db.execute("ROLLBACK;").await)
    {
        Ok(_) => Ok(()),
        Err(_) => Err("Error saving object to database"),
    }
}

pub async fn read_object(
    mut db: sqlx::pool::PoolConnection<sqlx::MySql>,
    id: &str,
) -> Result<Object, &'static str> {
    let mut rows = match db
        .fetch_all(sqlx::query!(
            "SELECT object.guid, object.name, object.description,
        GROUP_CONCAT(DISTINCT target.name SEPARATOR ',') AS targets,
        GROUP_CONCAT(DISTINCT language.name SEPARATOR ',') AS languages
        FROM object_application_objects AS object
        JOIN object_application_object_languages AS languages ON object.id = languages.object_id
        JOIN object_application_languages AS language ON language.id = languages.language_id
        JOIN object_application_object_targets AS targets ON object.id = targets.object_id
        JOIN object_application_targets AS target ON targets.target_id = target.id
        WHERE object.guid = ?;",
            id,
        ))
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

        match object_factory::restore_object(
            row.get("guid"),
            row.get("name"),
            row.get("description"),
            row.get::<&str, &str>("languages").split(",").collect(),
            row.get::<&str, &str>("targets").split(",").collect(),
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

async fn read_objects(
    mut db: sqlx::pool::PoolConnection<sqlx::MySql>,
    name: Option<&str>,
    targets: Option<Vec<&str>>,
    languages: Option<Vec<&str>>,
    keywords: Option<&str>,
) -> Result<Vec<Object>, &'static str> {
    let mut targets = match targets {
        Some(targets) => targets.join("','"),
        None => "".to_string(),
    };
    targets.insert_str(0, "'");
    targets.push('\'');

    let mut languages = match languages {
        Some(languages) => languages.join("','"),
        None => "".to_string(),
    };
    languages.insert_str(0, "'");
    languages.push('\'');

    let mut rows = match db
        .fetch_all(sqlx::query!(
            "SELECT object.guid, object.name, object.description,
        GROUP_CONCAT(DISTINCT target.name SEPARATOR ',') AS targets,
        GROUP_CONCAT(DISTINCT language.name SEPARATOR ',') AS languages
        FROM object_application_objects AS object
        JOIN object_application_object_languages AS languages ON object.id = languages.object_id
        JOIN object_application_languages AS language ON language.id = languages.language_id
        JOIN object_application_object_targets AS targets ON object.id = targets.object_id
        JOIN object_application_targets AS target ON targets.target_id = target.id
        WHERE (object.name LIKE '%?%' OR target.name IN (?) OR language.name IN (?)
        OR MATCH(`description`) AGAINST (''));",
            name,
            targets,
            //languages,
            //keywords,
        ))
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

        match object_factory::restore_object(
            row.get("guid"),
            row.get("name"),
            row.get("description"),
            row.get::<&str, &str>("languages").split(",").collect(),
            row.get::<&str, &str>("targets").split(",").collect(),
        ) {
            Ok(object) => objects.push(object),
            _ => (),
        }
    }

    Ok(objects)
}
