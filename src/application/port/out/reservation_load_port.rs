use async_trait::async_trait;

use crate::{domain::reservation::Reservation, dto::reservation_chk_dto::ReservationLimits};

#[async_trait]
pub trait ReservationLoadPort: Send + Sync {
    async fn load_reservation(&self, reservation_id: i32) -> Option<Reservation>;
    async fn load_reservations_by_user(&self, user_id: &str) -> Result<Vec<Reservation>, String>;
    async fn load_reservation_by_content_schedule(&self, content_schedule_id:u64)-> Result<Vec<Reservation>, String>;  
    async fn check_reservation_for_user_count(&self, user_id: &str, schedule_id: u64) -> Result<ReservationLimits, String>;
    async fn check_schedule_and_reservation(&self, user_id: &str, schedule_id: u64) -> Result<bool,String>;
    async fn check_user_reservation_for_content(&self, user_id: &str, schedule_id: u64) -> Result<bool, String>;
}
