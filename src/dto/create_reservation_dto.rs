use serde::Deserialize;
// ✅ CreateReservationRequest 구조체 (예약 생성 요청 - 예약 시간 포함)
#[derive(Debug, Deserialize)]
pub struct CreateReservationRequest {
    pub content_schedule_id: u64,
    pub reserved_at: Option<String>,
    pub ad_cnt: i32,
    pub cd_cnt: i32, 
}