use tonic::Request;
use user::{user_service_client::UserServiceClient, UserId, UserResponse};
use tonic::transport::Channel;
use auth::auth_service_client::AuthServiceClient;
use auth::ValidateTokenRequest;


pub mod auth {
    tonic::include_proto!("auth"); 
}

pub mod user {
    tonic::include_proto!("user"); 
}

pub struct GrpcClients {
    pub auth_client: AuthServiceClient<Channel>,
    pub user_client: UserServiceClient<Channel>,
}

impl GrpcClients {
    /// 새로운 gRPC 클라이언트 인스턴스를 생성
    pub async fn new(auth_addr: &str, user_addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let auth_client = AuthServiceClient::connect(auth_addr.to_string()).await?;
        let user_client = UserServiceClient::connect(user_addr.to_string()).await?;
        
        Ok(Self { auth_client, user_client })
    }

    /// 더미 클라이언트 (테스트용)
    pub fn dummy() -> Self {
        let auth_channel = Channel::from_static("http://localhost:50052").connect_lazy();
        let user_channel = Channel::from_static("http://localhost:50053").connect_lazy();
        
        Self {
            auth_client: AuthServiceClient::new(auth_channel),
            user_client: UserServiceClient::new(user_channel),
        }
    }

    /// 인증 토큰 검증 (Auth gRPC 호출)
    pub async fn validate_token(&mut self, token: String) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(ValidateTokenRequest { token });

        // 🔹 AuthServiceClient를 이용해 gRPC 요청
        let response = self.auth_client.validate_token(request).await?;
        let inner_response = response.into_inner();
        println!("🔹 Raw gRPC Response: {:?}", inner_response);
        
        // 🔹 빈 user_id 체크
        let user_id = if inner_response.user_id.trim().is_empty() {
            println!("⚠️ gRPC returned empty user_id! Treating as None.");
            None
        } else {
            println!("✅ gRPC Response received user_id: {}", inner_response.user_id);
            Some(inner_response.user_id)
        };

        Ok(user_id)
    }

    /// 사용자 정보 조회 (User gRPC 호출)
    pub async fn get_user_info(&mut self, user_id: String) -> Result<UserResponse, Box<dyn std::error::Error>> {
        let request = Request::new(UserId { random_id: user_id });

        // 🔹 UserService의 FindById gRPC 호출
        let response = self.user_client.find_by_id(request).await?;
        let user_info = response.into_inner();

        println!("User info received: {:?}", user_info);
        Ok(user_info)
    }
    
}