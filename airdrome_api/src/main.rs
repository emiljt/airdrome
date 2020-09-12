mod controllers;

use actix_web::{App, HttpServer, web};
use controllers::{objects_controller};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/objects")
                .route("", web::get().to(objects_controller::get_objects))
                .route("/{id}", web::get().to(objects_controller::get_object))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
