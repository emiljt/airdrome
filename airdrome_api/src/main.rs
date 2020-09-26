mod controllers;

use std::env;
use actix_web::{App, HttpServer, web};
use controllers::{objects_controller};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let db_pool = sqlx::MySqlPool::new(&db_url).await
        .expect("Error creating database connection pool");

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .service(web::scope("/objects")
                .route("", web::get().to(objects_controller::get_objects))
                .route("/{id}", web::get().to(objects_controller::get_object))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
