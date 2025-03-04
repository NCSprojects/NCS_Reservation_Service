use actix_web::web;
use std::sync::Arc;
use crate::state::AppState;
use crate::infra::web::reservation_controller::ReservationController;

pub fn configure(cfg: &mut web::ServiceConfig, state: Arc<AppState>) {
    let controller = state.reservation_controller.clone(); // ✅ AppState에서 컨트롤러 가져오기

    cfg.service(
        web::scope("/reservation")
            .route("/create", web::post().to(ReservationController::create_reservation))
            .route("/user",web::get().to(ReservationController::show_user_reservations))
            .route("/{id}", web::get().to(ReservationController::show_reservation))
            .route("/count",web::post().to(ReservationController::update_reservation))
            .route("/use", web::post().to(ReservationController::use_reservation))
            .route("/cancellation", web::post().to(ReservationController::cancel_reservation))
            .app_data(web::Data::new(controller.clone())),
    );
}
