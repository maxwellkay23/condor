#[derive(Debug, thiserror::Error)]
pub enum CondorError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Invalid job status: {0}")]
    InvalidJobStatus(String),
    #[error("{0}")]
    Generic(String),
}
