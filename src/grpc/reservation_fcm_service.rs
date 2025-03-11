use tonic::{Request, Response, Status};
use std::sync::Arc;
use async_trait::async_trait;

// ✅ 기존 서비스 포트 import
use crate::{application::port::out::reservation_load_port::ReservationLoadPort, reservationfcm_proto::{reservation_service_server::ReservationService, ContentScheduleRequest, UserList}};

pub struct ReservationFcmGrpcService {
    reservation_port: Arc<dyn ReservationLoadPort + Send + Sync>,  
}

impl ReservationFcmGrpcService {
    pub fn new(reservation_port: Arc<dyn ReservationLoadPort + Send + Sync>) -> Self {
        ReservationFcmGrpcService { reservation_port }
    }
}

#[async_trait]
impl ReservationService for ReservationFcmGrpcService {
    async fn get_users_by_content_schedule_id(
        &self,
        request: Request<ContentScheduleRequest>,
    ) -> Result<Response<UserList>, Status> {
        let req = request.into_inner();
        println!("Received request for ContentScheduleId: {}", req.content_schedule_id);
        //타입 변환
        let content_schedule_id: u64 = match req.content_schedule_id.parse() {
            Ok(id) => id,
            Err(_) => return Err(Status::invalid_argument("Invalid content_schedule_id format")),
        };
        
        let reservations = match self.reservation_port.load_reservations_by_content_schedule(content_schedule_id).await {
            Ok(reservations) => reservations,
            Err(err) => return Err(Status::internal(format!("Failed to load reservations: {}", err))),
        };

        // user_id 추출
        let user_ids: Vec<String> = reservations.into_iter().map(|r| r.user_id.to_string()).collect();

        let response = UserList { user_ids };

        Ok(Response::new(response))
    }
}