use actix_web::{Responder};

pub async fn get_objects() -> impl Responder {
    "get_objects"
}