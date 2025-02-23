use std::sync::Arc;

use async_trait::async_trait;

use crate::{application::port::out::{reservation_load_port::ReservationLoadPort, reservation_save_port::ReservationSavePort}, domain::reservation::{Reservation, ReservationStatus}, dto::reservation_chk_dto::ReservationLimits, infra::db::reservation_repository::{ReservationRepository, ReservationRepositoryImpl}};

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
    async fn check_reservation_for_user_count(&self, user_id: &str, schedule_id: u64) -> Result<ReservationLimits, String> {
        let limits = self.repository.check_reservation_for_user_count(user_id, schedule_id).await?;
        Ok(limits) 
    }
    async fn check_schedule_and_reservation(&self, user_id: &str, schedule_id: u64) -> Result<bool, String> {
        let chk_val = self.repository.check_schedule_and_reservation(user_id, schedule_id).await?;
        Ok(chk_val) 
    }
    async fn check_user_reservation_for_content(&self, user_id: &str, schedule_id: u64) -> Result<bool, String>{
        let chk_val = self.repository.check_user_reservation_for_content(user_id, schedule_id).await?;
        Ok(chk_val) 
    }
    async fn load_reservations_by_user(&self, user_id: &str) -> Result<Vec<Reservation>, String>
    {
        self.repository.load_reservations_by_user(user_id).await
    }
    
}

#[async_trait]
impl ReservationSavePort for ReservationAdapter {
    async fn save_reservation(&self, reservation: Reservation) -> Result<(), String> {
        self.repository.save_reservation(reservation).await
    }
    async fn update_status(&self, reservation_id: i32, status: ReservationStatus) -> Result<(), String> {
        self.repository.update_status(reservation_id,status).await
    }
    async fn update_reservaiton_user_count(&self, reservation_id: i32, ad_cnt:i32, cd_cnt:i32) -> Result<(), String>
    {
        self.repository.update_reservaiton_user_count(reservation_id, ad_cnt, cd_cnt).await
    }
}

