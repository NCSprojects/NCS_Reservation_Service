use actix_web::web;
use crate::adapter::in_web::reservation_controller;

// `/reservation` 경로 설정
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reservation") // 기본 경로 `/reservation`
            .route("/hi", web::get().to(reservation_controller::say_hi)) // `/reservation/hi`
    );
}
