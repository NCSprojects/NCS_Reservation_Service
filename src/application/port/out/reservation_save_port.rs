use async_trait::async_trait;

use crate::domain::reservation::Reservation;

#[async_trait]
pub trait ReservationSavePort: Send + Sync {
    async fn save_reservation(&self, reservation: Reservation) -> Result<(), String>;
}