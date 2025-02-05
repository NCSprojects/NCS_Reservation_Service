use std::sync::Arc;

use async_trait::async_trait;

use crate::{adapter::reservation_adapter::ReservationAdapter, domain::reservation::Reservation};

use super::port::{r#in::reservation_usecase::ReservationUseCase, out::{reservation_load_port::ReservationLoadPort, reservation_save_port::ReservationSavePort}};

// Use Case Implementation
pub struct ReservationService {
    adapter: Arc<ReservationAdapter>,
}

impl ReservationService {
    pub fn new(adapter: Arc<ReservationAdapter>) -> Self {
        Self { adapter }
    }
}

#[async_trait]
impl ReservationUseCase for ReservationService {
    async fn create_reservation(&self, reservation: Reservation) -> Result<(), String> {
        self.adapter.save_reservation(reservation).await
    }

    async fn show_reservation(&self, reservation_id: i32) -> Result<Reservation, String> {
        self.adapter
            .load_reservation(reservation_id)
            .await
            .ok_or("Reservation not found".to_string())
    }

    // async fn update_status(&self, reservation_id: i32, status: ReservationStatus) -> Result<(), String> {
    //     self.adapter.update_status(reservation_id, status).await
    // }

    // async fn delete_reservation(&self, reservation_id: i32) -> Result<(), String> {
    //     self.adapter.delete_reservation(reservation_id).await
    // }
}
