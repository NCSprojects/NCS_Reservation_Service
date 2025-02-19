pub mod state;
pub mod settings;
pub mod startup;
pub mod infra;
pub mod grpc_client; 
pub mod r#struct;
pub mod db_connection;
pub mod grpc_server;
pub mod application;
pub mod domain;
pub mod adapter;
pub mod dto;
pub mod grpc;
pub mod error;
pub mod reservation_proto {
    tonic::include_proto!("reservation"); 
}
