use async_trait::async_trait;

use crate::{domain::reservation::{Reservation, ReservationStatus}, dto::reservation_chk_dto::ReservationLimits};

#[async_trait]
pub trait ReservationRepository: Send + Sync {
    async fn load_reservation(&self, reservation_id: i32) -> Option<Reservation>;
    async fn load_reservations_by_user(&self, user_id: &str) -> Result<Vec<Reservation>, String>; 
    async fn load_reservation_by_content_schedule(&self, content_schedule_id:u64) -> Result<Vec<Reservation>,String>;
    async fn save_reservation(&self, reservation: Reservation) -> Result<(), String>;
    async fn update_status(&self, reservation_id: i32, status: ReservationStatus) -> Result<(), String>;
    async fn update_reservaiton_user_count(&self, reservation_id: i32, ad_cnt:i32, cd_cnt:i32) -> Result<(), String>;
    async fn delete_reservation(&self, reservation_id: i32) -> Result<(), String>;
    async fn check_reservation_for_user_count(&self, user_id: &str, schedule_id: u64) -> Result<ReservationLimits, String>;
    async fn check_schedule_and_reservation(&self, user_id: &str, schedule_id: u64) -> Result<bool,String>;
    async fn check_user_reservation_for_content(&self, user_id: &str, schedule_id: u64) -> Result<bool, String>;
}