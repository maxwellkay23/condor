#[derive(Debug, thiserror::Error)]
pub enum CondorServerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Address parse error: {0}")]
    NetAddr(#[from] std::net::AddrParseError),
    #[error("Transport error: {0}")]
    TonicTransport(#[from] tonic::transport::Error),
    #[error("Invalid job status: {0}")]
    InvalidJobStatus(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

impl From<condor_common::CondorError> for CondorServerError {
    fn from(e: condor_common::CondorError) -> Self {
        match e {
            condor_common::CondorError::InvalidJobStatus(s) => {
                CondorServerError::InvalidJobStatus(s)
            }
            other => CondorServerError::InvalidJobStatus(other.to_string()),
        }
    }
}
