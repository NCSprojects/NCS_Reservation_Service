use async_trait::async_trait;

use crate::domain::reservation::Reservation;

#[async_trait]
pub trait ReservationUseCase: Send + Sync {
    async fn create_reservation(&self, reservation: Reservation) -> Result<(), String>;
    async fn show_reservation(&self, reservation_id: i32 )->  Result<Reservation, String>; 
}