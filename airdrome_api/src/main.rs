mod controllers;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use controllers::objects_controller;
use event_application;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let db_pool = sqlx::MySqlPool::connect(&db_url)
        .await
        .expect("Error creating database connection pool");
    let event_tx = event_application::create_event_thread();

    event_tx
        .send(event_application::Event::ServerStarted)
        .expect("Unable to send server started event");

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(event_tx.clone())
            .service(
                web::scope("/objects")
                    .route("", web::get().to(objects_controller::get_objects))
                    .route("/{id}", web::get().to(objects_controller::get_object)),
            )
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
