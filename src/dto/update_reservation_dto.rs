use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateReservationRequest {
    pub reservation_id: i32,
    pub ad_cnt: i32,
    pub cd_cnt: i32,
}