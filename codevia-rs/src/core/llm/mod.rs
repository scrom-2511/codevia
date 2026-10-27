mod api_keys_provider;
mod client;
mod gemini_sdk;

pub use api_keys_provider::*;
pub use client::*;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    #[error("Missing API key: GEMINI_API_KEY environment variable is not set")]
    MissingApiKey,
    #[error("Missing model environment variable '{0}' and no fallback was found")]
    MissingModel(String),
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API error (status {status}): {message}")]
    ApiError {
        status: reqwest::StatusCode,
        message: String,
    },
    #[error("Stream error: {0}")]
    StreamError(String),
}

pub struct LLMResponse {
    pub output_text: String,
    pub response: Value,
}

impl LLMResponse {
    pub fn new(output_text: impl Into<String>, response: Value) -> Self {
        Self {
            output_text: output_text.into(),
            response,
        }
    }
}
