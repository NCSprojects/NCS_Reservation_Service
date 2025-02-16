use tonic::transport::Channel;
use auth::auth_service_client::AuthServiceClient;
use auth::{ValidateTokenRequest, ValidateTokenResponse};

pub mod auth {
    tonic::include_proto!("auth"); 
}

pub struct AuthGrpcClient {
    client: AuthServiceClient<Channel>,
}

impl AuthGrpcClient {
    pub async fn new(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client: AuthServiceClient<Channel> = AuthServiceClient::connect(addr.to_string()).await?;
        Ok(Self { client })
    }

    pub fn dummy() -> Self {
        let channel = tonic::transport::Channel::from_static("http://localhost:50052").connect_lazy();
        Self {
            client: AuthServiceClient::new(channel),
        }
    }

    pub async fn validate_token(&mut self, token: String) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(ValidateTokenRequest { token });
    
        // 🔹 Call gRPC method
        let response = self.client.validate_token(request).await?;
        let inner_response = response.into_inner();
        println!("🔹 Raw gRPC Response: {:?}", inner_response);
        
        // 🔹 Convert empty user_id to `None`
        let user_id = if inner_response.user_id.trim().is_empty() {
            println!("⚠️ gRPC returned empty user_id! Treating as None.");
            None
        } else {
            println!("✅ gRPC Response received user_id: {}", inner_response.user_id);
            Some(inner_response.user_id)
        };
    
        Ok(user_id)
    }
}