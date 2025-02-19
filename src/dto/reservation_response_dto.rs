use serde::Serialize;
use crate::domain::reservation::Reservation;

// ✅ ReservationDTO 구조체 (API 응답용)
#[derive(Debug, Serialize)]
pub struct ReservationDTO {
    pub id: i32,
    pub user_id: String,
    pub content_schedule_id: u64,
    pub reserved_at: Option<String>,
    pub ad_cnt: i32,
    pub cd_cnt: i32, 
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
            ad_cnt: reservation.ad_cnt,
            cd_cnt: reservation.cd_cnt,
            use_at: reservation.use_at,
        }
    }
}