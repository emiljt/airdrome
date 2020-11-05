mod applications;
mod controllers;
mod services;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use applications::obex_application;
use controllers::objects_controller;
// use event_application;
// use events_service::Event;
use actix_web::middleware::Logger;
use env_logger::Env;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let db_pool = sqlx::MySqlPool::connect(&db_url)
        .await
        .expect("Error creating database connection pool");
    // let event_tx = event_application::create_event_thread();

    // event_tx
    //     .send(Event::ServerStarted {
    //         temp_path: "/tmp/obex".to_string(),
    //     })
    //     .expect("Unable to send server started event");
    // obex_application::sync(db_pool.clone(), "/tmp/obex").await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(db_pool.clone())
            // .data(event_tx.clone())
            .service(
                web::scope("/api/objects")
                    .route("", web::get().to(objects_controller::get_objects))
                    .route("/{id}", web::get().to(objects_controller::get_object)),
            )
            .service(
                web::scope("/api/health").route("/heartbeat", web::get().to(|| HttpResponse::Ok())),
            )
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}
