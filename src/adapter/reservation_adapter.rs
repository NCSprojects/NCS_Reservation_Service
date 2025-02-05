use std::sync::Arc;

use async_trait::async_trait;

use crate::{application::port::out::{reservation_load_port::ReservationLoadPort, reservation_save_port::ReservationSavePort}, domain::reservation::Reservation, infra::db::reservation_repository::{ReservationRepository, ReservationRepositoryImpl}};

// Adapter Implementation
pub struct ReservationAdapter {
    repository: Arc<ReservationRepositoryImpl>,
}

impl ReservationAdapter {
    pub fn new(repository: Arc<ReservationRepositoryImpl>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ReservationLoadPort for ReservationAdapter {
    async fn load_reservation(&self, reservation_id: i32) -> Option<Reservation> {
        self.repository.load_reservation(reservation_id.try_into().unwrap()).await
    }
}

#[async_trait]
impl ReservationSavePort for ReservationAdapter {
    async fn save_reservation(&self, reservation: Reservation) -> Result<(), String> {
        self.repository.save_reservation(reservation).await
    }
}