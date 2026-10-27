use async_stream::stream;
use futures::{Stream, StreamExt};
use reqwest::{Client, Response};
use serde_json::Value;

use crate::core::llm::{LLMError, LLMResponse};

pub struct GeminiSDK {
    client: Client,
    base_url: String,
    api_key: String,
}

pub struct InteractionConfig<'a> {
    pub input: &'a str,
    pub model: Option<&'a str>,
    pub tools: Option<&'a Value>,
    pub response_format: Option<&'a Value>,
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

impl GeminiSDK {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        let base_url =
            String::from("https://generativelanguage.googleapis.com/v1beta/interactions");

        Self {
            base_url,
            api_key,
            client,
        }
    }

    pub async fn interaction_response(
        &self,
        config: InteractionConfig<'_>,
        stream: bool,
    ) -> Result<Response, LLMError> {
        let input = config.input;

        let model = config.model.unwrap_or_else(|| "gemini-3.5-flash-lite");

        let tools = config.tools;
        let response_format = config.response_format;

        let mut body = serde_json::json!({
            "input": input,
            "model": model,
        });

        if stream {
            body["stream"] = serde_json::json!(true);
        }

        if let Some(response_format) = response_format {
            body["response_format"] = serde_json::json!(response_format);
        }

        if let Some(tools) = tools {
            body["tools"] = serde_json::json!(tools);
        }

        let url = format!("{}", self.base_url);

        let res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("X-Goog-Api-Key", self.api_key.as_str())
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let message = res.text().await.unwrap_or_default();
            return Err(LLMError::ApiError { status, message });
        }

        Ok(res)
    }

    pub async fn create_interaction(
        &self,
        config: InteractionConfig<'_>,
    ) -> Result<LLMResponse, LLMError> {
        let res = self.interaction_response(config, false).await?;

        let response_json: Value = res.json().await?;
        let output_text = self.extract_output_text(&response_json);

        Ok(LLMResponse {
            output_text,
            response: response_json,
        })
    }

    pub async fn create_interaction_stream(
        &self,
        config: InteractionConfig<'_>,
    ) -> Result<impl Stream<Item = Result<String, LLMError>> + use<>, LLMError> {
        let res = self.interaction_response(config, true).await?;

        let mut response_stream = res.bytes_stream();

        let output = stream! {
            while let Some(chunk_result) = response_stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        match String::from_utf8(chunk.to_vec()) {
                            Ok(text) => yield Ok(text),
                            Err(err) => {
                                yield Err(LLMError::StreamError(err.to_string()));
                                break;
                            }
                        }
                    }
                    Err(err) => {
                        yield Err(LLMError::from(err));
                        break;
                    }
                }
            }
        };

        Ok(output)
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
