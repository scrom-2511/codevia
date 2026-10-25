use async_stream::stream;
use futures::{Stream, StreamExt};
use serde_json::Value;

use crate::core::llm::{LLMError, types::LLMResponse};

pub struct GeminiSDK<'a> {
    base_url: &'a str,
    api_key: &'a str,
}

pub struct InteractionConfig<'a> {
    pub input: &'a str,
    pub model: Option<&'a str>,
    pub tools: Option<&'a str>,
    pub response_format: Option<&'a str>,
}

impl<'a> Default for InteractionConfig<'a> {
    fn default() -> Self {
        Self {
            input: "",
            model: None,
            tools: None,
            response_format: None,
        }
    }
}

impl<'a> GeminiSDK<'a> {
    pub fn new(base_url: &'a str, api_key: &'a str) -> Self {
        Self { base_url, api_key }
    }

    pub async fn create_interaction<'b>(
        &self,
        config: InteractionConfig<'b>,
    ) -> Result<LLMResponse, LLMError> {
        let input = config.input;
        let model = config.model.unwrap_or_else(|| "gemini-3.5-flash");
        let tools = config
            .tools
            .map(|t| serde_json::to_string(&t))
            .transpose()?;
        let response_format = config.response_format;

        let mut body = serde_json::json!({
            "input": input,
            "model": model,
        });

        if let Some(response_format) = response_format {
            body["response_format"] = serde_json::json!(response_format);
        }

        if let Some(tools) = tools {
            body["tools"] = serde_json::json!(tools);
        }

        let client = reqwest::Client::new();
        let url = format!("{}", self.base_url);

        let res = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("X-Goog-Api-Key", self.api_key)
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let message = res.text().await.unwrap_or_default();
            return Err(LLMError::ApiError { status, message });
        }

        let response_json: Value = res.json().await?;
        let output_text = self.extract_output_text(&response_json);

        Ok(LLMResponse {
            output_text,
            response: response_json,
        })
    }

    pub fn extract_output_text(&self, response: &Value) -> String {
        if let Some(text) = response.get("output_text").and_then(|v| v.as_str()) {
            if !text.is_empty() {
                return text.to_string();
            }
        }

        let mut parts = Vec::new();

        if let Some(steps) = response.get("steps").and_then(|v| v.as_array()) {
            for step in steps {
                if let Some(content) = step.get("content").and_then(|v| v.as_array()) {
                    for item in content {
                        if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                            parts.push(text);
                        }
                    }
                } else if let Some(content_str) = step.get("content").and_then(|v| v.as_str()) {
                    parts.push(content_str);
                }

                if let Some(text) = step.get("text").and_then(|v| v.as_str()) {
                    parts.push(text);
                }
            }
        }

        parts.join("")
    }
}
