use chrono::{DateTime, Utc};
use sqlx::{query, MySqlPool,Row};
use async_trait::async_trait;
use std::sync::Arc;
use std::str::FromStr; 
use crate::{domain::reservation::{Reservation, ReservationStatus}, dto::reservation_chk_dto::ReservationLimits,infra::db::reservation_repository::ReservationRepository};

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
            "SELECT id, user_id, content_schedule_id, reserved_at, ad_cnt, cd_cnt, status, use_at FROM RESERVATION WHERE id = ?",
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
                ad_cnt: row.ad_cnt.unwrap_or(0),
                cd_cnt: row.cd_cnt.unwrap_or(0), 
                status: row.status.and_then(|s| ReservationStatus::from_str(&s).ok()),
                use_at: row.use_at != 0,
            })
        } else {
            None
        }
    }
    async fn laod_reservations_by_date(&self, start_time: DateTime<Utc>, end_time:DateTime<Utc>) -> Result<Vec<Reservation>,String>
    {
        let rows = query(
            "
             SELECT 
                id, user_id, content_schedule_id, reserved_at, status, ad_cnt, cd_cnt, use_at  
             FROM RESERVATION
             where reserved_at BETWEEN ? AND ?  
            "
        )
        .bind(start_time)
        .bind(end_time)
        .fetch_all(&*self.pool)
        .await
        .map_err(|err| err.to_string())?;

        let reservations = rows.into_iter().map(|row| {
            let status: Option<String> = row.try_get("status").ok();
            let status = status.and_then(|s| match s.as_str() {
                "PENDING" => Some(ReservationStatus::Pending),
                "CONFIRMED" => Some(ReservationStatus::Confirmed),
                "CANCELLED" => Some(ReservationStatus::Cancelled),
                _ => None,
            });

            Reservation {
                id: row.get("id"),
                user_id: row.get("user_id"),
                content_schedule_id: row.get("content_schedule_id"),
                reserved_at: row.try_get("reserved_at").ok(),
                status,
                ad_cnt: row.get("ad_cnt"),
                cd_cnt: row.get("cd_cnt"),
                use_at: row.get::<i8, _>("use_at") != 0, // `TINYINT(1)` → `bool` 변환
            }
        }).collect();

        Ok(reservations)
    }

    async fn load_reservations_by_user(&self, user_id: &str) -> Result<Vec<Reservation>, String> 
    {
        let rows = query(
            "
            SELECT 
                id, user_id, content_schedule_id, reserved_at, status, ad_cnt, cd_cnt, use_at 
            FROM RESERVATION
            WHERE user_id = ?
            "
        )
        .bind(user_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|err| err.to_string())?;

        let reservations = rows.into_iter().map(|row| {
            let status: Option<String> = row.try_get("status").ok();
            let status = status.and_then(|s| match s.as_str() {
                "PENDING" => Some(ReservationStatus::Pending),
                "CONFIRMED" => Some(ReservationStatus::Confirmed),
                "CANCELLED" => Some(ReservationStatus::Cancelled),
                _ => None,
            });

            Reservation {
                id: row.get("id"),
                user_id: row.get("user_id"),
                content_schedule_id: row.get("content_schedule_id"),
                reserved_at: row.try_get("reserved_at").ok(),
                status,
                ad_cnt: row.get("ad_cnt"),
                cd_cnt: row.get("cd_cnt"),
                use_at: row.get::<i8, _>("use_at") != 0, // `TINYINT(1)` → `bool` 변환
            }
        }).collect();

        Ok(reservations)
    }
    async fn load_reservations_by_content_schedule(&self, content_schedule_id:u64)-> Result<Vec<Reservation>, String> {
        let rows = query(
            "
            SELECT 
                id, user_id, content_schedule_id, reserved_at, status, ad_cnt, cd_cnt, use_at 
            FROM RESERVATION
            WHERE content_schedule_id = ?
            "
        ).bind(content_schedule_id)
        .fetch_all(&*self.pool)
        .await
        .map_err(|err| err.to_string())?;

        let reservations = rows.into_iter().map(|row| {
            let status: Option<String> = row.try_get("status").ok();
            let status = status.and_then(|s| match s.as_str() {
                "PENDING" => Some(ReservationStatus::Pending),
                "CONFIRMED" => Some(ReservationStatus::Confirmed),
                "CANCELLED" => Some(ReservationStatus::Cancelled),
                _ => None,
            });

            Reservation {
                id: row.get("id"),
                user_id: row.get("user_id"),
                content_schedule_id: row.get("content_schedule_id"),
                reserved_at: row.try_get("reserved_at").ok(),
                status,
                ad_cnt: row.get("ad_cnt"),
                cd_cnt: row.get("cd_cnt"),
                use_at: row.get::<i8, _>("use_at") != 0, // `TINYINT(1)` → `bool` 변환
            }
        }).collect();

        Ok(reservations)
    }

    async fn save_reservation(&self, reservation: Reservation) -> Result<(), String> {
        let status_str = reservation.status.map(|s| s.to_string());
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
    
        // 현재 예약 인원 조회
        let schedule_data = sqlx::query!(
            "SELECT c.tot_seats AS total_seats, cs.adult_count, cs.child_count
             FROM CONTENT_SCHEDULES cs
             JOIN CONTENTS c ON c.id = cs.content_id
             WHERE cs.id = ?",
            reservation.content_schedule_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    
        let total_seats: i32 = schedule_data.total_seats.unwrap_or(0);
        let current_adults: i32 = schedule_data.adult_count;
        let current_children: i32 = schedule_data.child_count;
    
        let new_total = reservation.ad_cnt + reservation.cd_cnt; // 새롭게 추가할 예약 인원
        let final_total = current_adults + current_children + new_total; // 최종 예약 후 인원
    
        println!(
            "total_seats: {:?}, 현재 예약된 인원(스케줄): {:?}, 새로 예약할 인원: {:?}, 최종 인원: {:?}",
            total_seats, current_adults + current_children, new_total, final_total
        );
    
        // total_seats 초과 확인
        if final_total > total_seats {
            tx.rollback().await.map_err(|e| e.to_string())?;
            return Err(format!(
                "예약 불가: 최대 좌석 수 초과 (최대 {:?}명, 현재 예약 {:?}명, 요청한 예약 {:?}명)",
                total_seats, current_adults + current_children, new_total
            ));
        }
    
        // `INSERT` 실행 (최대 좌석을 초과하지 않을 경우)
        sqlx::query!(
            "INSERT INTO RESERVATION (user_id, content_schedule_id, reserved_at, ad_cnt, cd_cnt, status, use_at) 
             VALUES (?, ?, NOW(), ?, ?, ?, ?)",
            reservation.user_id,
            reservation.content_schedule_id,
            reservation.ad_cnt,
            reservation.cd_cnt,
            status_str.as_deref(),
            reservation.use_at as i8
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query!(
            "UPDATE CONTENT_SCHEDULES
             SET adult_count = adult_count + ?, 
                 child_count = child_count + ?
             WHERE id = ?",
            reservation.ad_cnt,
            reservation.cd_cnt,
            reservation.content_schedule_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn update_status(&self, reservation_id: i32, status: ReservationStatus) -> Result<(), String> {
        let status_str = status.to_string(); 
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
    // 인원 수 수정
    async fn update_reservaiton_user_count(&self, reservation_id: i32, ad_cnt:i32, cd_cnt:i32) -> Result<(), String>{
        
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        let reservation_data = sqlx::query!(
            "SELECT content_schedule_id, ad_cnt, cd_cnt 
             FROM RESERVATION 
             WHERE id = ?",
            reservation_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
        
        let schedule_id: u64 = reservation_data.content_schedule_id;
        let current_reservation_adults: i32 = reservation_data.ad_cnt.unwrap_or(0); //  NULL 값 처리
        let current_reservation_children: i32 = reservation_data.cd_cnt.unwrap_or(0);

        // 현재 예약 인원 조회
        let schedule_data = sqlx::query!(
            "SELECT c.tot_seats AS total_seats, cs.adult_count, cs.child_count
             FROM CONTENT_SCHEDULES cs
             JOIN CONTENTS c ON c.id = cs.content_id
             WHERE cs.id = ?",
             schedule_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    
        let total_seats: i32 = schedule_data.total_seats.map(|v| v as i32).unwrap_or(0);
        let current_adults: i32 = schedule_data.adult_count;
        let current_children: i32 = schedule_data.child_count;
        
        let new_total = ad_cnt + cd_cnt; // 새롭게 추가할 예약 인원
        let final_total = current_adults + current_children + new_total - (current_reservation_adults+ current_reservation_children); // 최종 예약 후 인원

        // total_seats 초과 확인
            if final_total > total_seats {
            tx.rollback().await.map_err(|e| e.to_string())?;
            return Err(format!(
                "예약 불가: 최대 좌석 수 초과 (최대 {:?}명, 현재 예약 {:?}명, 요청한 예약 {:?}명)",
                total_seats, current_adults + current_children, new_total
            ));
        }
        
         // 예약 정보 업데이트 (RESERVATION 테이블)
    sqlx::query!(
        "UPDATE RESERVATION
         SET ad_cnt = ?, 
             cd_cnt = ?
         WHERE id = ?",
        ad_cnt,
        cd_cnt,
        reservation_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

        // 스케줄 정보 업데이트 
        sqlx::query!(
            "UPDATE CONTENT_SCHEDULES
            SET adult_count = adult_count - ? + ?, 
                child_count = child_count - ? + ?
            WHERE id = ?",
            current_reservation_adults, ad_cnt,
            current_reservation_children, cd_cnt,
            schedule_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?; 
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
    async fn check_reservation_for_user_count(&self, user_id: &str, schedule_id: u64) -> Result<ReservationLimits, String> {
        let mut tx: sqlx::Transaction<'_, sqlx::MySql> = self.pool.begin().await.map_err(|e| e.to_string())?;
        
        // 예약 가능 여부 조회 
        /* 총원 수 검사*/
        let result = sqlx::query_as::<_, ReservationLimits>(
            "WITH content_info AS (
                SELECT content_id FROM CONTENT_SCHEDULES WHERE id = ?
            )
            SELECT 
                COALESCE(CAST(SUM(re.ad_cnt) AS SIGNED), 0) AS total_adults,
                COALESCE(CAST(SUM(re.cd_cnt) AS SIGNED), 0) AS total_children
            FROM RESERVATION re
            JOIN CONTENT_SCHEDULES cs ON re.content_schedule_id = cs.id
            JOIN content_info ci ON cs.content_id = ci.content_id
            WHERE re.user_id = ?
            AND re.status != 'CANCELED'"
        )
        .bind(schedule_id)
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await
        .or_else(|err| {
            if err.to_string().contains("no rows returned") {
                println!("데이터 없음 → 모든 값 `-1`로 설정하여 반환");
                Ok(ReservationLimits {
                    total_adults: Some(-1),
                    total_children: Some(-1),
                })
            } else {
                Err(err.to_string())
            }
        })?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(result)
    }
    
    // 동일 시간대에 대한 예약 건이 있는지 확인
    async fn check_schedule_and_reservation(&self,  user_id: &str, schedule_id: u64
    ) -> Result<bool, String> {
        let has_reservation: bool = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 
                FROM RESERVATION r
                JOIN CONTENT_SCHEDULES cs ON r.content_schedule_id = cs.id
                WHERE cs.start_time = (
                    SELECT start_time FROM CONTENT_SCHEDULES WHERE id = ?
                )
                AND r.user_id = ?
                AND (r.status IS NULL OR r.status != 'CANCELED')
            ) AS has_reservation;
            "#,
            schedule_id,
            user_id
        )
        .fetch_one(&*self.pool)
        .await
        .map(|row| row.has_reservation != 0) // `1`이면 true, `0`이면 false
        .map_err(|e| e.to_string())?;

        Ok(has_reservation)
    }

    // 동일 컨텐츠에 대한 예약 건이 있는지 확인
    async fn check_user_reservation_for_content(&self, user_id: &str, schedule_id: u64) -> Result<bool, String> {
        let has_reservation: bool = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 
                FROM RESERVATION r
                JOIN CONTENT_SCHEDULES cs ON r.content_schedule_id = cs.id
                WHERE cs.content_id = (SELECT content_id FROM CONTENT_SCHEDULES WHERE id = ?)
                AND r.user_id = ?
                AND r.status != 'CANCELED' -- 취소된 예약 제외
            ) AS has_reservation;
            "#,
            schedule_id,
            user_id
        )
        .fetch_one(&*self.pool)  
        .await
        .map(|row| row.has_reservation != 0)  // `1`이면 true, `0`이면 false
        .map_err(|e| e.to_string())?;
    
        Ok(has_reservation)
    }
}
