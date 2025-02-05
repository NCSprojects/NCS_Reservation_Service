use std::{fmt, str::FromStr};

use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone ,FromRow)]
pub struct Reservation {
    pub id: i32,
    pub user_id: i32,
    pub content_schedule_id: i32,
    pub reserved_at: Option<DateTime<Utc>>,
    pub status: Option<ReservationStatus>,  
    pub use_at: bool,
}



#[derive(Debug, Clone)]
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

