use std::sync::Arc;

use async_trait::async_trait;

use crate::{common::date::get_today_start_end_date, domain::reservation::{Reservation, ReservationStatus}, dto::reservation_chk_dto::ReservationLimits};

use super::port::{r#in::reservation_usecase::ReservationUseCase, out::{reservation_load_port::ReservationLoadPort, reservation_save_port::ReservationSavePort}};

// Use Case Implementation
pub struct ReservationService {
    save_port: Arc<dyn ReservationSavePort + Send + Sync>,
    load_port: Arc<dyn ReservationLoadPort + Send + Sync>,
}

impl ReservationService {
    pub fn new(save_port: Arc<dyn ReservationSavePort + Send + Sync>, load_port: Arc<dyn ReservationLoadPort + Send + Sync>) -> Self {
        Self { save_port, load_port }
    }
    fn is_reservation_available(&self, count: ReservationLimits, max_adult: i32, max_child: i32) -> bool {
        // 로그: 입력값 출력
        println!(
            "is_reservation_available 호출됨 | total_adults: {:?}, total_children: {:?}, max_adult: {}, max_child: {}",
            count.total_adults, count.total_children, max_adult, max_child
        );
    
        // 타입 변환 후 값
        let total_adults = count.total_adults.map(|v| v as i64).unwrap_or(-1);
        let total_children = count.total_children.map(|v| v as i64).unwrap_or(-1);
    
        // 로그: 변환된 값 출력
        println!(
            "변환된 값 | total_adults: {}, total_children: {}",
            total_adults, total_children
        );
    
        // 인원 초과 체크
        if total_adults > max_adult.into() {
            println!("예약 불가: 성인 수 초과 ({}명 > {}명)", total_adults, max_adult);
            return false;
        }
        if total_children > max_child.into() {
            println!("예약 불가: 어린이 수 초과 ({}명 > {}명)", total_children, max_child);
            return false;
        }
        true
    }
    /// 예약 입력값을 검증하는 함수
    fn validate_reservation_input_count(&self, ad_cnt: i32, cd_cnt: i32, max_adult: i32, max_child: i32) -> bool {
        // 성인 인원 초과 검사
        if ad_cnt > max_adult && ad_cnt > 0{
            return  false;
        }

        // 어린이 인원 초과 검사
        if cd_cnt > max_child && cd_cnt > 0{
            return false;
        }
        true
    }
}

#[async_trait]
impl ReservationUseCase for ReservationService {
    async fn create_reservation(&self, reservation: Reservation) -> Result<(), String> {
        self.save_port.save_reservation(reservation).await
    }

    async fn show_reservation(&self, reservation_id: i32) -> Result<Reservation, String> {
        self.load_port
            .load_reservation(reservation_id)
            .await
            .ok_or("Reservation not found".to_string())
    }

    async fn show_today_reservations(&self) -> Result<Vec<Reservation>,String> {
        let (start_time, end_time) = get_today_start_end_date();
         println!("Start Time: {}", start_time);
        println!("End Time: {}", end_time);
        self.load_port
            .load_reservations_by_date(start_time, end_time)
            .await
    }

    async fn show_user_reservations(&self, user_id: &str) -> Result<Vec<Reservation>,String>{
        self.load_port
            .load_reservations_by_user(user_id)
            .await
    }

    async fn check_reservation(&self, user_id: String, schedule_id: u64, ad_cnt: i32, cd_cnt: i32, max_adult:i32,max_child:i32) -> Result<bool, String> {
        // 사용자 입력 데이터 검증
        if !self.validate_reservation_input_count(ad_cnt, cd_cnt, max_adult, max_child) {
            return Ok(false);
        }

        let dup_reservation = self.load_port.check_user_reservation_for_content(&user_id, schedule_id).await?;
        if dup_reservation
        {
            let limits = self.load_port.check_reservation_for_user_count(&user_id, schedule_id).await?;

            // Private 함수 호출
            if !self.is_reservation_available(limits, max_adult, max_child) {
                return Ok(false);
            }
        }
    
        Ok(true) // 예약 가능 → true 반환
    }
    
    //예약 사용하기 
    async fn use_reservation(&self, reservation_id: i32) -> Result<(), String> {
        let use_status = ReservationStatus::Confirmed;
        self.save_port.update_status(reservation_id, use_status).await
    }

    //예약 취소하기
    async fn cancel_reservation(&self, reservation_id: i32) -> Result<(), String> {
        let cancel_status = ReservationStatus::Cancelled;
        self.save_port.update_status(reservation_id, cancel_status).await
    }

    //예약 수정하기
    async fn update_reservation(&self, reservation_id: i32, ad_cnt: i32, cd_cnt: i32, max_adult: i32, max_child: i32) -> Result<(), String> {  
        // 사용자 입력 데이터 검증
        if !self.validate_reservation_input_count(ad_cnt, cd_cnt, max_adult, max_child) {
        return Err("예약 불가".to_string());
        }

        let my_reservation = self.load_port.load_reservation(reservation_id).await;

        if let Some(reservation) = my_reservation {  
            let schedule_id = reservation.content_schedule_id;  
            let user_id = reservation.user_id.clone(); 


            let chk_val = match self.load_port.check_reservation_for_user_count(&user_id, schedule_id).await {
                Ok(val) => {
                    val
                }
                Err(e) => {
                    return Err(format!("현재 예약 인원 조회 실패: {}", e));
                }
            };

            // 예약 가능 여부 체크
            if !self.is_reservation_available(chk_val, max_adult, max_child) {
                return Err("예약 불가: 인원 초과".to_string()); 
            }

            // 인원 업데이트 실행
            match self.save_port.update_reservaiton_user_count(reservation_id, ad_cnt, cd_cnt).await {
                Ok(_) => println!("예약 인원 업데이트 성공"),
                Err(e) => {
                    return Err(format!("예약 인원 업데이트 실패: {}", e));
                }
            };
            Ok(()) 

    } else {
        Err(format!("예약을 찾을 수 없습니다! ID: {}", reservation_id)) 
    }
    }

    // async fn delete_reservation(&self, reservation_id: i32) -> Result<(), String> {
    //     self.adapter.delete_reservation(reservation_id).await
    // }
}
