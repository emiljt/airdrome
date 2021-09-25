use super::object_application;
use crate::services::events_service::Event;
use log::{debug, error, info, warn};
use obex::obex::Obex;
use std::path::Path;
use std::sync::mpsc;

pub async fn sync(db_pool: &sqlx::Pool<sqlx::MySql>, path: &str) {
    let path = Path::new(path);
    let obex = Obex::new(
        "https://github.com/parallaxinc/propeller.git",
        &path
            .to_str()
            .expect("Unable able to convert string to path"),
    )
    .expect("Unable to create obex");
    let obex_version = obex.version();

    for object in &obex.official_categories {}

    for object in &obex.community_categories {}

    // for object in &obex.official_objects {
    //     let mut db_connection = db_pool
    //         .acquire()
    //         .await
    //         .expect("Unable to connect to database");

    //     match object_application::add_new_object(
    //         db_connection,
    //         &object.name,
    //         "",
    //         Vec::new(),
    //         Vec::new(),
    //     )
    //     .await
    //     {
    //         Ok(o) => println!("Added new object from obex: {}", o.name),
    //         Err(_) => (),
    //     };
    // }

    'new_objects: for object in &obex.community_objects {
        info!("Processing object {}", &object.name);

        let object_path = Path::new(&object.path);

        // Check if object already exists in database
        let existing_objects = object_application::search_objects(
            &db_pool,
            Some(&object.name),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;

        // let mut existing_object = None::<object_application::Object>;

        if existing_objects.len() > 0 {
            info!("Found {} matching objects", existing_objects.len());

            for duplicate in existing_objects {
                if duplicate.name == object.name {
                    info!(
                        "Found existing object, attempting to update {}",
                        object.name
                    );

                    match object_application::update_object(
                        &db_pool,
                        &duplicate.id,
                        "",
                        Vec::new(),
                        Vec::new(),
                        object_path,
                        None,
                        Some(&obex_version),
                    )
                    .await
                    {
                        Ok(_) => info!("Updated {}", &object.name),
                        Err(e) => error!("{}", e),
                    }
                    continue 'new_objects;
                } else {
                    info!("{} didn't match", duplicate.name);
                }
            }

            warn!("None of the existing objects matched");
        }

        match object_application::add_new_object(
            &db_pool,
            &object.name,
            "",
            Vec::new(),
            Vec::new(),
            object_path,
            None,
            Some(&obex_version),
        )
        .await
        {
            Ok(o) => info!("Added new object from obex: {}", o.name),
            Err(e) => error!("{}", e),
        };
    }
}

pub struct ObexObject {
    name: String,
    path: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
