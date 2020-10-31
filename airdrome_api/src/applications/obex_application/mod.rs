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

    for object in &obex.community_objects {
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
