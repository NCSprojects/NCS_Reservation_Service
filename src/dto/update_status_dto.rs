use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateStatusRequest {
    pub reservation_id: i32
}