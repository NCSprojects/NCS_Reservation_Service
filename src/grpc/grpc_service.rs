use tonic::{Request, Response, Status};
use std::sync::Arc;
use crate::{application::port::r#in::reservation_usecase::ReservationUseCase, domain::reservation::Reservation, reservation_proto::{reservation_service_server::ReservationService, CreateReservationRequest, CreateReservationResponse}};

pub struct ReservationGrpcService {
    reservation_service: Arc<dyn ReservationUseCase + Send + Sync>,  
}

#[tonic::async_trait]
impl ReservationService for ReservationGrpcService {
    async fn create_reservation(
        &self,
        request: Request<CreateReservationRequest>,
    ) -> Result<Response<CreateReservationResponse>, Status> {
        let req = request.into_inner();
        println!("Received reservation request: {:?}", req);

         
        let reservation: Reservation = req.into();
        // 기존 서비스 (`ReservationService`) 사용
        let result = self.reservation_service.create_reservation(reservation).await;

        let response = match result {
            Ok(_) => CreateReservationResponse {
                success: true,
                message: "Reservation created successfully".to_string(),
            },
            Err(err) => CreateReservationResponse {
                success: false,
                message: format!("Failed to create reservation: {}", err),
            },
        };

        Ok(Response::new(response))
    }
}

impl ReservationGrpcService {
    pub fn new(reservation_service: Arc<dyn ReservationUseCase + Send + Sync>) -> Self {
        ReservationGrpcService { reservation_service }
    }
}