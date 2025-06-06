use reqwest::StatusCode;
use std::{error, fmt};

#[derive(Debug)]
pub enum OctopustError {
    Api(ApiError),
    // ... other error variants ...
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    // Add other variants as needed
}

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl fmt::Display for OctopustError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OctopustError::Api(err) => write!(f, "API Error ({}): {}", err.status, err.message),
            OctopustError::Reqwest(e) => write!(f, "Request error: {}", e),
            OctopustError::Serde(e) => write!(f, "Serialization error: {}", e),
            // Add display for other variants as needed
        }
    }
}

impl error::Error for OctopustError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            OctopustError::Reqwest(e) => Some(e),
            OctopustError::Serde(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for OctopustError {
    fn from(e: reqwest::Error) -> Self {
        OctopustError::Reqwest(e)
    }
}

impl From<serde_json::Error> for OctopustError {
    fn from(e: serde_json::Error) -> Self {
        OctopustError::Serde(e)
    }
}