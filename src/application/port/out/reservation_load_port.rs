use async_trait::async_trait;

use crate::domain::reservation::Reservation;

#[async_trait]
pub trait ReservationLoadPort: Send + Sync {
    async fn load_reservation(&self, reservation_id: i32) -> Option<Reservation>;
}
