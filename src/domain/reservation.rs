use std::{fmt, str::FromStr};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::prelude::{FromRow, Type};

use crate::reservation_proto::CreateReservationRequest;

#[derive(Debug, Clone ,FromRow)]
pub struct Reservation {
    pub id: i32,
    pub user_id: String,
    pub content_schedule_id: u64,
    pub reserved_at: Option<DateTime<Utc>>,
    pub status: Option<ReservationStatus>, 
    pub ad_cnt: i32,
    pub cd_cnt: i32, 
    pub use_at: bool,
}

impl Reservation {
    pub fn is_valid_capacity(&self, new_ad_cnt: i32, new_cd_cnt: i32) -> bool {
        new_ad_cnt >= self.ad_cnt && new_cd_cnt >= self.cd_cnt
    }
}

#[derive(Debug, Clone, Type, Deserialize)]
#[sqlx(type_name = "VARCHAR")] 
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Cancelled,
}
impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            ReservationStatus::Pending => "PENDING",
            ReservationStatus::Confirmed => "CONFIRMED",
            ReservationStatus::Cancelled => "CANCELLED",
        };
        write!(f, "{}", status_str)
    }
}
impl From<String> for ReservationStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Pending" => Self::Pending,
            "Confirmed" => Self::Confirmed,
            "Cancelled" => Self::Cancelled,
            _ => Self::Pending, 
        }
    }
}
impl FromStr for ReservationStatus {
    type Err = ();

    fn from_str(status: &str) -> Result<Self, Self::Err> {
        match status {
            "PENDING" => Ok(Self::Pending),
            "CONFIRMED" => Ok(Self::Confirmed),
            "CANCELLED" => Ok(Self::Cancelled),
            _ => Err(()),
        }
    }
}

// gRPC CreateReservationRequest를 Reservation으로 변환
impl From<CreateReservationRequest> for Reservation {
    fn from(req: CreateReservationRequest) -> Self {
        Reservation {
            id: 0,
            user_id: req.user_id,
            content_schedule_id: req.content_schedule_id,
            reserved_at: None,
                // .as_deref() // Option<String> → Option<&str>
                // .and_then(|s| s.parse::<DateTime<Utc>>().ok()), // ✅ 변환 시도 (실패하면 None)
            ad_cnt: req.ad_cnt,
            cd_cnt: req.cd_cnt,
            status: None,
            use_at: false,
        }
    }
}
