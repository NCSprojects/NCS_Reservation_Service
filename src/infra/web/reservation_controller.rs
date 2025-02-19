use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc, ParseError};
use crate::application::port::r#in::reservation_usecase::ReservationUseCase;
use crate::domain::reservation::{Reservation, ReservationStatus};
use crate::dto::create_reservation_dto::CreateReservationRequest;
use crate::dto::update_reservation_dto::UpdateReservationRequest;
use crate::dto::reservation_response_dto::ReservationDTO;
use crate::grpc_client::GrpcClients; 

// `String` → `DateTime<Utc>` 변환 함수
fn parse_datetime_from_string(datetime_str: &str) -> Result<DateTime<Utc>, ParseError> {
    DateTime::parse_from_rfc3339(datetime_str).map(|dt| dt.with_timezone(&Utc))
}

#[derive(Clone)]
pub struct ReservationController {
    use_case: Arc<dyn ReservationUseCase + Send + Sync>,
    grpc_clients: Arc<Mutex<GrpcClients>>, 
}

impl ReservationController {
    pub fn new(
        use_case: Arc<dyn ReservationUseCase + Send + Sync>, 
        grpc_clients: Arc<Mutex<GrpcClients>> 
    ) -> Self {
        Self { use_case, grpc_clients }
    }

    // ✅ `/reservation/hi` 엔드포인트 (테스트용)
    pub async fn say_hi() -> impl Responder {
        HttpResponse::Ok().body("hi")
    }

    // 예약 생성
    pub async fn create_reservation(
        controller: web::Data<Arc<ReservationController>>,
        req: web::Json<CreateReservationRequest>,
        http_req: HttpRequest, 
    ) -> impl Responder {
        // 헤더에서 JWT 가져오기
        let token = match http_req.headers().get("Authorization") {
            Some(value) => value.to_str().unwrap_or("").replace("Bearer ", "").trim().to_string(),
            None => return HttpResponse::Unauthorized().json("No Authorization Header"),
        };
    
        // gRPC를 사용하여 AuthService에 토큰 검증 요청
        let mut grpc_clients = controller.grpc_clients.lock().await;
        let user_id = match grpc_clients.validate_token(token).await {
            Ok(Some(user_id)) => {
                println!("✅ Received user_id from gRPC: {}", user_id);
                user_id
            },
            Ok(None) => {
                println!("⚠️ gRPC returned None for user_id!");
                return HttpResponse::Unauthorized().json("Invalid Token");
            },
            Err(err) => {
                println!("❌ gRPC call failed: {}", err);
                return HttpResponse::InternalServerError().json("Auth Service Error");
            }
        };
        println!("Received user_id from Auth Service: {}", user_id);
        /* userId로 USer-service로 통신해서 User 정보 가져오기*/
        let user_info = match grpc_clients.get_user_info(user_id.clone()).await {
            Ok(user) => {
                println!("✅ User-Service returned user info: {:?}", user);
                user
            },
            Err(err) => {
                println!("❌ gRPC call to User-Service failed: {}", err);
                return HttpResponse::InternalServerError().json("User Service Error");
            }
        };
        // 예약 가능 여부 확인
        let is_available = match controller.use_case.check_reservation(user_id.clone(), req.content_schedule_id, req.ad_cnt,req.cd_cnt, user_info.ad_cnt,user_info.cd_cnt).await {
            Ok(true) => true,
            Ok(false) => return HttpResponse::BadRequest().json("🚫 예약 불가: 인원 초과 또는 중복 예약 불가"),
            Err(e) => return HttpResponse::InternalServerError().json(format!("예약 검증 실패: {}", e)),
        };
    
        // 예약 객체 생성
        if is_available {
            let reservation = Reservation {
                id: 0,
                user_id, 
                content_schedule_id: req.content_schedule_id,
                reserved_at: None,
                ad_cnt: req.ad_cnt,
                cd_cnt: req.cd_cnt,
                status: Some(ReservationStatus::Pending),
                use_at: false,
            };
    
            // 예약 생성 처리
            match controller.use_case.create_reservation(reservation).await {
                Ok(_) => HttpResponse::Created().json("✅ 예약이 성공적으로 생성되었습니다."),
                Err(e) => HttpResponse::InternalServerError().json(format!("❌ 예약 생성 실패: {}", e)),
            }
        } else {
            HttpResponse::BadRequest().json("🚫 예약 불가: 인원 초과 또는 중복 예약 불가")
        }
    }

    // /reservation/{id} 엔드포인트 - 예약 조회 (DTO 반환)
    pub async fn show_reservation(
        controller: web::Data<Arc<ReservationController>>,
        reservation_id: web::Path<i32>,
    ) -> impl Responder {
        match controller.use_case.show_reservation(reservation_id.into_inner()).await {
            Ok(reservation) => HttpResponse::Ok().json(ReservationDTO::from(reservation)), 
            Err(e) => HttpResponse::NotFound().json(format!("Error: {}", e)),
        }
    }

    // /reservation - 예약 수정
    pub async fn update_reservation (
        controller: web::Data<Arc<ReservationController>>,
        req: web::Json<UpdateReservationRequest>, 
        http_req: HttpRequest,
    ) -> impl Responder {
        let token = match http_req.headers().get("Authorization") {
            Some(value) => value.to_str().unwrap_or("").replace("Bearer ", "").trim().to_string(),
            None => return HttpResponse::Unauthorized().json("No Authorization Header"),
        };
    
        // gRPC를 사용하여 AuthService에 토큰 검증 요청
        let mut grpc_clients = controller.grpc_clients.lock().await;
        let user_id = match grpc_clients.validate_token(token).await {
            Ok(Some(user_id)) => {
                println!("Received user_id from gRPC: {}", user_id);
                user_id
            },
            Ok(None) => {
                println!("gRPC returned None for user_id!");
                return HttpResponse::Unauthorized().json("Invalid Token");
            },
            Err(err) => {
                println!("gRPC call failed: {}", err);
                return HttpResponse::InternalServerError().json("Auth Service Error");
            }
        };
    
        // 유저 정보 가져오기
        let user_info = match grpc_clients.get_user_info(user_id.clone()).await {
            Ok(info) => {
                println!("User info received: {:?}", info);
                info
            },
            Err(e) => {
                println!("Failed to fetch user info: {}", e);
                return HttpResponse::InternalServerError().json(format!("Failed to get user info: {}", e));
            }
        };
    
        // 🔹 DTO에서 필요한 정보 추출
        let reservation_id = req.reservation_id;
        let ad_cnt = req.ad_cnt;
        let cd_cnt = req.cd_cnt;
    
        match controller.use_case.update_reservation(reservation_id, ad_cnt, cd_cnt,user_info.ad_cnt,user_info.cd_cnt).await {
            Ok(_) => HttpResponse::Ok().json("✅ 예약이 성공적으로 수정되었습니다."),
            Err(e) => HttpResponse::InternalServerError().json(format!("❌ 예약 수정 실패: {}", e)),
        }
    }
}