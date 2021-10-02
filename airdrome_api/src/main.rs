mod applications;
mod controllers;
mod services;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use applications::obex_application;
use controllers::objects_controller;
// use event_application;
// use events_service::Event;
use actix_files::Files;
use actix_web::middleware::Logger;
use env_logger::Env;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let domain = env::var("DOMAIN").expect("DOMAIN environment variable not set");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let obex_path = env::var("OBEX_PATH").expect("OBEX_PATH environment variable not set");
    let static_path = env::var("STATIC_PATH").expect("STATIC_PATH environment variable not set");
    let db_pool = sqlx::MySqlPool::connect(&db_url)
        .await
        .expect("Error creating database connection pool");
    // let event_tx = event_application::create_event_thread();

    // event_tx
    //     .send(Event::ServerStarted {
    //         temp_path: "/tmp/obex".to_string(),
    //     })
    //     .expect("Unable to send server started event");
    obex_application::sync(&db_pool, &obex_path).await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("Access-Control-Allow-Origin", "api.{domain}"),
            )
            // .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            // .wrap(middleware::Compress::default())
            .data(db_pool.clone())
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
