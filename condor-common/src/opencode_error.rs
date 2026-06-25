use progenitor_client::Error as ProgenitorError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CondorOpencodeError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },
    #[error("Invalid response payload: {message} (parse error: {parse_error})")]
    InvalidResponse {
        message: String,
        parse_error: String,
    },
    #[error("{0}")]
    InvalidRequest(String),
}

impl From<crate::openapi::types::error::ConversionError> for CondorOpencodeError {
    fn from(e: crate::openapi::types::error::ConversionError) -> Self {
        CondorOpencodeError::ApiError {
            status: 400,
            message: e.to_string(),
        }
    }
}

impl<T: std::fmt::Debug> From<ProgenitorError<T>> for CondorOpencodeError {
    fn from(e: ProgenitorError<T>) -> Self {
        match e {
            ProgenitorError::InvalidRequest(msg) => CondorOpencodeError::InvalidRequest(msg),
            ProgenitorError::CommunicationError(e) => CondorOpencodeError::Http(e),
            ProgenitorError::InvalidUpgrade(e) => CondorOpencodeError::Http(e),
            ProgenitorError::ErrorResponse(rv) => CondorOpencodeError::ApiError {
                status: rv.status().as_u16(),
                message: format!("{:?}", rv.into_inner()),
            },
            ProgenitorError::ResponseBodyError(e) => CondorOpencodeError::Http(e),
            ProgenitorError::InvalidResponsePayload(bytes, json_err) => {
                CondorOpencodeError::InvalidResponse {
                    message: String::from_utf8_lossy(&bytes).to_string(),
                    parse_error: json_err.to_string(),
                }
            }
            ProgenitorError::UnexpectedResponse(r) => CondorOpencodeError::ApiError {
                status: r.status().as_u16(),
                message: format!("unexpected response: {}", r.status()),
            },
            ProgenitorError::Custom(msg) => CondorOpencodeError::ApiError {
                status: 0,
                message: msg,
            },
        }
    }
}
