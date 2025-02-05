use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, ParseError};
use crate::application::port::r#in::reservation_usecase::ReservationUseCase;
use crate::domain::reservation::Reservation;

// ✅ ReservationRequest 구조체 (예약 생성 요청)
#[derive(Debug, Deserialize)]
pub struct ReservationRequest {
    pub user_id: i32,
    pub content_schedule_id: i32,
    pub use_at: bool,
}

// ✅ CreateReservationRequest 구조체 (예약 생성 요청 - 예약 시간 포함)
#[derive(Debug, Deserialize)]
pub struct CreateReservationRequest {
    pub user_id: i32,
    pub content_schedule_id: i32,
    pub reserved_at: Option<String>, 
    pub use_at: bool,
}

// ✅ ReservationDTO 구조체 (API 응답용)
#[derive(Debug, Serialize)]
pub struct ReservationDTO {
    pub id: i32,
    pub user_id: i32,
    pub content_schedule_id: i32,
    pub reserved_at: Option<String>,
    pub use_at: bool,
}

// ✅ Reservation → ReservationDTO 변환 함수
impl From<Reservation> for ReservationDTO {
    fn from(reservation: Reservation) -> Self {
        ReservationDTO {
            id: reservation.id,
            user_id: reservation.user_id,
            content_schedule_id: reservation.content_schedule_id,
            reserved_at: reservation.reserved_at.map(|dt| dt.to_rfc3339()), // ✅ `DateTime<Utc>`를 `String`으로 변환
            use_at: reservation.use_at,
        }
    }
}

// ✅ `String` → `DateTime<Utc>` 변환 함수
fn parse_datetime_from_string(datetime_str: &str) -> Result<DateTime<Utc>, ParseError> {
    DateTime::parse_from_rfc3339(datetime_str)
        .map(|dt| dt.with_timezone(&Utc))
}

// ✅ ReservationController 구조체
#[derive(Clone)]
pub struct ReservationController {
    use_case: Arc<dyn ReservationUseCase + Send + Sync>,
}

impl ReservationController {
    pub fn new(use_case: Arc<dyn ReservationUseCase + Send + Sync>) -> Self {
        Self { use_case }
    }

    // ✅ `/reservation/hi` 엔드포인트 (테스트용)
    pub async fn say_hi() -> impl Responder {
        HttpResponse::Ok().body("hi")
    }

    // ✅ `/reservation/create` 엔드포인트 - 예약 생성
    pub async fn create_reservation(
        controller: web::Data<Arc<ReservationController>>,
        req: web::Json<CreateReservationRequest>,
    ) -> impl Responder {
        let reserved_at = req
            .reserved_at
            .as_deref()
            .map_or(Ok(None), |dt| parse_datetime_from_string(dt).map(Some));

        let reservation = Reservation {
            id: 0,
            user_id: req.user_id,
            content_schedule_id: req.content_schedule_id,
            reserved_at: match reserved_at {
                Ok(reserved_at) => reserved_at,
                Err(_) => return HttpResponse::BadRequest().json("Invalid datetime format"),
            },
            status: None,
            use_at: req.use_at,
        };

        match controller.use_case.create_reservation(reservation).await {
            Ok(_) => HttpResponse::Created().json("Reservation successfully created"),
            Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        }
    }

    // ✅ `/reservation/{id}` 엔드포인트 - 예약 조회 (DTO 반환)
    pub async fn show_reservation(
        controller: web::Data<Arc<ReservationController>>,
        reservation_id: web::Path<i32>,
    ) -> impl Responder {
        match controller.use_case.show_reservation(reservation_id.into_inner()).await {
            Ok(reservation) => HttpResponse::Ok().json(ReservationDTO::from(reservation)), 
            Err(e) => HttpResponse::NotFound().json(format!("Error: {}", e)),
        }
    }
}
