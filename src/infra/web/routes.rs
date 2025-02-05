use actix_web::web;
use std::sync::Arc;
use crate::state::AppState;
use crate::infra::web::reservation_controller::ReservationController;

pub fn configure(cfg: &mut web::ServiceConfig, state: Arc<AppState>) {
    let controller = state.reservation_controller.clone(); // ✅ AppState에서 컨트롤러 가져오기

    cfg.service(
        web::scope("/reservation")
            .route("/hi", web::get().to(ReservationController::say_hi))
            .route("/create", web::post().to(ReservationController::create_reservation))
            .route("/{id}", web::get().to(ReservationController::show_reservation))
            .app_data(web::Data::new(controller.clone())),
    );
}
