use serde::Deserialize;

// ✅ ReservationRequest 구조체 (예약 생성 요청)
#[derive(Debug, Deserialize)]
pub struct ReservationRequest {
    pub user_id: String,
    pub content_schedule_id: u64,
    pub use_at: bool,
}