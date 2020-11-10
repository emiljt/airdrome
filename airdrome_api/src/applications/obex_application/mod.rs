use super::object_application;
use crate::services::events_service::Event;
use obex::obex::Obex;
use std::path::Path;
use std::sync::mpsc;

pub async fn sync(db_pool: sqlx::Pool<sqlx::MySql>, path: &str) {
    let path = Path::new(path);
    let obex = Obex::new(
        "https://github.com/parallaxinc/propeller.git",
        &path
            .to_str()
            .expect("Unable able to convert string to path"),
    )
    .expect("Unable create obex");

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
        let mut db_connection = db_pool
            .acquire()
            .await
            .expect("Unable to connect to database");

        // Check if object already exists in database
        let existing_objects = object_application::search_objects(
            db_connection,
            Some(&object.name),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;

        let mut existing_object = None::<object_application::Object>;

        if existing_objects.len() > 0 {
            for duplicate in existing_objects {
                if duplicate.name == object.name {
                    println!("Found existing object, skipping: {}", object.name);
                    continue 'new_objects;
                }
            }
        }

        let mut db_connection = db_pool
            .acquire()
            .await
            .expect("Unable to connect to database");

        match object_application::add_new_object(
            db_connection,
            &object.name,
            "",
            Vec::new(),
            Vec::new(),
        )
        .await
        {
            Ok(o) => println!("Added new object from obex: {}", o.name),
            Err(_) => (),
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
