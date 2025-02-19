use std::io;

#[derive(Debug)]
pub enum ServerError {
    Io(io::Error),
    Tonic(tonic::transport::Error),
}

impl From<io::Error> for ServerError {
    fn from(error: io::Error) -> Self {
        ServerError::Io(error)
    }
}

impl From<tonic::transport::Error> for ServerError {
    fn from(error: tonic::transport::Error) -> Self {
        ServerError::Tonic(error)
    }
}