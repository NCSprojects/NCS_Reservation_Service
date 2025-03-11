use async_trait::async_trait;

use crate::domain::reservation::Reservation;

#[async_trait]
pub trait ReservationUseCase: Send + Sync {
    async fn create_reservation(&self, reservation: Reservation) -> Result<(), String>;
    async fn show_reservation(&self, reservation_id: i32 )->  Result<Reservation, String>;
    async fn show_user_reservations(&self, user_id:&str) -> Result<Vec<Reservation>,String>; 
    async fn show_today_reservations(&self) -> Result<Vec<Reservation>,String>;   
    async fn check_reservation(&self, user_id: String,schedule_id: u64, ad_cnt: i32, cd_cnt: i32, max_adult:i32,max_child:i32) -> Result<bool, String>; 
    async fn use_reservation(&self, reservation_id: i32 ) -> Result<(), String>;
    async fn cancel_reservation(&self, reservation_id: i32) -> Result<(), String>;
    async fn update_reservation(&self, reservation_id: i32, ad_cnt: i32, cd_cnt: i32, max_adult: i32, max_child: i32) -> Result<(), String>;
}