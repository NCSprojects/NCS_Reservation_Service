use async_trait::async_trait;

use crate::domain::reservation::{Reservation, ReservationStatus};

#[async_trait]
pub trait ReservationSavePort: Send + Sync {
    async fn save_reservation(&self, reservation: Reservation) -> Result<(), String>;
    async fn update_status(&self, reservation_id: i32, status: ReservationStatus) -> Result<(), String>;
    async fn update_reservaiton_user_count(&self, reservation_id: i32, ad_cnt:i32, cd_cnt:i32) -> Result<(), String>;
}