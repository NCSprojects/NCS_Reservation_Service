use actix_web::{HttpResponse, Responder};

// `/hi` 경로에 대한 핸들러
pub async fn say_hi() -> impl Responder {
    HttpResponse::Ok().body("hi")
}

