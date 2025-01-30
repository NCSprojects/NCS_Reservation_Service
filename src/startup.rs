use actix_web::{web, App, HttpServer, middleware::Logger};
use tokio::{task, time::{sleep, Duration}};
use reqwest::Client;
use std::net::TcpListener;
use crate::{infra::web::routes::configure, settings::Settings, r#struct::eureka_info::{DataCenterInfo, EurekaDetails, EurekaInfo, EurekaPortDetails}};
use crate::state::AppState;
use std::sync::Arc;

async fn run_eureka_client(state: Arc<AppState>) {
    let client = Client::new();
    let settings = Arc::clone(&state).settings.clone();  // Arc 내부 데이터 가져오기

    println!("Instance ID: {}", settings.instance_id);
    println!("Eureka Server: {}", settings.eureka_server);
    println!("App Name: {}", settings.app_name);

     // ✅ EurekaInfo 구조체 생성
     let instance_info = EurekaInfo {
        instance: EurekaDetails {
            instance_id: settings.instance_id.clone(),
            host_name: "127.0.0.1".to_string(),
            app: settings.app_name.clone(),
            ip_addr: "127.0.0.1".to_string(),
            vip_address: settings.app_name.clone(),
            status: "UP".to_string(),
            port: EurekaPortDetails {
                port: settings.server_port,
                enabled: "true".to_string(),
            },
            data_center_info: DataCenterInfo {
                class: "com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo".to_string(),
                name: "MyOwn".to_string(),
            },
        },
    };

    let register_url = format!("{}/apps/{}", settings.eureka_server, settings.app_name);
    match client.post(&register_url).json(&instance_info).send().await {
        Ok(response) if response.status().is_success() => {
            println!("Successfully registered with Eureka!");
        }
        Ok(response) => {
            eprintln!("Failed to register. Status: {}", response.status());
        }
        Err(err) => {
            eprintln!("Eureka registration error: {}", err);
        }
    }

    let heartbeat_url = format!("{}/apps/{}/{}", settings.eureka_server, settings.app_name, settings.instance_id);
    loop {
        match client.put(&heartbeat_url).send().await {
            Ok(response) if response.status().is_success() => println!("Heartbeat sent."),
            Ok(response) => eprintln!("Heartbeat failed. Status: {}", response.status()),
            Err(err) => eprintln!("Error sending heartbeat: {}", err),
        }
        sleep(Duration::from_secs(30)).await;
    }
}

pub fn run(listener: TcpListener, state: Arc<AppState>) -> Result<actix_web::dev::Server, std::io::Error> {
    // Eureka 클라이언트 실행 (비동기 태스크)
    task::spawn(run_eureka_client(state.clone()));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .configure(configure)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
