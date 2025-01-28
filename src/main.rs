mod r#struct;
mod adapter;

use actix_web::{web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use r#struct::eureka_info::{DataCenterInfo, EurekaDetails, EurekaInfo, EurekaPortDetails};
use reqwest::Client;
use tokio::{task, time::{sleep, Duration}};
use adapter::in_web::routes;

// Actix Web 헬스체크 핸들러
async fn health_check() -> impl Responder {
    web::Json(serde_json::json!({ "status": "UP" }))
}

// Actix Web 기본 핸들러
async fn index() -> impl Responder {
    "Hello, Actix Web with Eureka!"
}

// Eureka 클라이언트 관리 함수
async fn run_eureka_client(eureka_server: String, app_name: String, instance_id: String) {
    let client = Client::new();

    // 애플리케이션 등록
    let instance_info: EurekaInfo = EurekaInfo {
        instance: EurekaDetails {
            instance_id: instance_id.to_string(),
            host_name: "127.0.0.1".to_string(),
            app: app_name.clone(),
            ip_addr: "127.0.0.1".to_string(),
            vip_address: app_name.clone(),
            status: "UP".to_string(),
            port: EurekaPortDetails {
                port: 3200,
                enabled: "true".to_string(),
            },
            data_center_info: DataCenterInfo {
                class: "com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo".to_string(),
                name: "MyOwn".to_string(),
            },
        },
    };

    let register_url = format!("{}/apps/{}", eureka_server, app_name);
    match client.post(&register_url).json(&instance_info).send().await {
        Ok(response) if response.status().is_success() => {
            println!("Successfully registered with Eureka!");
        }
        Ok(response) => {
            eprintln!(
                "Failed to send register. Status: {}, Body: {:?}",
                response.status(),
                response.text().await.unwrap_or_else(|_| "No Body".to_string())
            );
        }
        Err(err) => {
            eprintln!("Error registering with Eureka: {}", err);
        }
    }

    // Heartbeat 전송 루프
    let heartbeat_url = format!("{}/apps/{}/{}", eureka_server, app_name, instance_id);
    loop {
        match client.put(&heartbeat_url).send().await {
            Ok(response) if response.status().is_success() => {
                println!("Heartbeat sent successfully.");
            }
            Ok(response) => {
                eprintln!(
                    "Failed to send heartbeat. Status: {}, Body: {:?}",
                    response.status(),
                    response.text().await.unwrap_or_else(|_| "No Body".to_string())
                );
            }
            Err(err) => {
                eprintln!("Error sending heartbeat: {}", err);
            }
        }

        // 30초마다 Heartbeat 전송
        sleep(Duration::from_secs(30)).await;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Eureka 설정
    let eureka_server = "http://localhost:8673/eureka".to_string();
    let app_name = "RESERVATION".to_string();
    let instance_id = "RESERVATION-APP:127.0.0.1:3200".to_string();

    // Eureka 클라이언트 비동기 실행
    task::spawn(run_eureka_client(eureka_server, app_name, instance_id));

    // Actix Web 서버 실행
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // 요청 로깅
            .configure(routes::configure)  // 기본 핸들러
    })
    .bind("127.0.0.1:3200")?
    .run()
    .await
}
