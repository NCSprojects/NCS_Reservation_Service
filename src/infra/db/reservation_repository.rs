use sqlx::MySqlPool;
use async_trait::async_trait;
use std::sync::Arc;
use std::str::FromStr;  // ✅ 추가
use crate::domain::reservation::{Reservation, ReservationStatus};

#[async_trait]
pub trait ReservationRepository: Send + Sync {
    async fn load_reservation(&self, reservation_id: i32) -> Option<Reservation>;
    async fn save_reservation(&self, reservation: Reservation) -> Result<(), String>;
    async fn update_status(&self, reservation_id: i32, status: ReservationStatus) -> Result<(), String>;
    async fn delete_reservation(&self, reservation_id: i32) -> Result<(), String>;
}

// Repository Implementation
pub struct ReservationRepositoryImpl {
    pool: Arc<MySqlPool>,
}

impl ReservationRepositoryImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReservationRepository for ReservationRepositoryImpl {
    async fn load_reservation(&self, reservation_id: i32) -> Option<Reservation> {
        if let Some(row) = sqlx::query!(
            "SELECT id, user_id, content_schedule_id, reserved_at, status, use_at FROM RESERVATION WHERE id = ?",
            reservation_id
        )
        .fetch_optional(&*self.pool)
        .await
        .ok()
        .flatten() {
            Some(Reservation {
                id: row.id,
                user_id: row.user_id,
                content_schedule_id: row.content_schedule_id,
                reserved_at: row.reserved_at,
                status: row.status.and_then(|s| ReservationStatus::from_str(&s).ok()), // ✅ 변경됨
                use_at: row.use_at != 0,
            })
        } else {
            None
        }
    }

    async fn save_reservation(&self, reservation: Reservation) -> Result<(), String> {
        let status_str = reservation.status.map(|s| s.to_string());
        sqlx::query!(
            "INSERT INTO RESERVATION (user_id, content_schedule_id, reserved_at, status, use_at) VALUES (?, ?, ?, ?, ?)",
            reservation.user_id,
            reservation.content_schedule_id,
            reservation.reserved_at,
            status_str.as_deref(),   
            reservation.use_at as i8
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn update_status(&self, reservation_id: i32, status: ReservationStatus) -> Result<(), String> {
        let status_str = status.to_string(); // ✅ 임시 값 방지
        sqlx::query!(
            "UPDATE RESERVATION SET status = ? WHERE id = ?",
            status_str,
            reservation_id
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn delete_reservation(&self, reservation_id: i32) -> Result<(), String> {
        sqlx::query!(
            "DELETE FROM RESERVATION WHERE id = ?",
            reservation_id
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
