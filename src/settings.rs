use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub eureka_server: String,
    pub app_name: String,
    pub instance_id: String,
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String, 
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config").required(false)) // config.toml 파일 읽기
            .add_source(Environment::with_prefix("APP").separator("__")) // 환경 변수 로드
            .build()?; // 설정 빌드

        config.try_deserialize() // 구조체로 변환
    }
}
