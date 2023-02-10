mod applications;
mod controllers;
mod services;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use applications::obex_application;
use controllers::objects_controller;
// use event_application;
// use events_service::Event;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use env_logger::Env;
use services::database::connection::{DatabaseConnection, DatabaseType};
use std::env;
use std::fs::create_dir_all;

#[derive(Clone)]
enum Environment {
    Dev,
    Prod,
    Stage,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment = match env::var("ENV").unwrap_or(String::from("prod")).as_str() {
        "prod" => Environment::Prod,
        "dev" => Environment::Dev,
        _ => Environment::Prod,
    };

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL variable not set");
    let mut db = DatabaseConnection::new(DatabaseType::Sqlite);

    db.connect(&db_uri).await.expect("Error opening database");

    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let obex_path = env::var("OBEX_PATH").expect("OBEX_PATH environment variable not set");
    let static_path = env::var("STATIC_PATH").expect("STATIC_PATH environment variable not set");
    //let db_pool = sqlx::MySqlPool::connect(&db_url)
    //    .await
    //    .expect("Error creating database connection pool");

    create_dir_all(&static_path).expect("Unable to create static files directory");

    // let event_tx = event_application::create_event_thread();

    // event_tx
    //     .send(Event::ServerStarted {
    //         temp_path: "/tmp/obex".to_string(),
    //     })
    //     .expect("Unable to send server started event");
    obex_application::sync(&db.clone_pool(), &obex_path).await;

    HttpServer::new(move || {
        let cors = match &environment {
            Environment::Dev => Cors::permissive(),
            _ => Cors::default(),
        };

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            // .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            // .wrap(middleware::Compress::default())
            .data(db.clone_pool())
            // .data(event_tx.clone())
            .service(
                web::scope("/objects")
                    .route("", web::get().to(objects_controller::get_objects))
                    .route(
                        "/{object_id}",
                        web::get().to(objects_controller::get_object),
                    )
                    .route(
                        "/{object_id}/versions/{version_id}/{file_type}",
                        web::get().to(objects_controller::get_object_version_file),
                    ),
            )
            .service(Files::new("/static", &static_path))
            .service(
                web::scope("/health").route("/heartbeat", web::get().to(|| HttpResponse::Ok())),
            )
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}
