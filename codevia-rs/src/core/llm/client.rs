use futures::Stream;

use crate::core::{
    conversations::ChatMessage,
    llm::{
        LLMError,
        gemini_sdk::{GeminiSDK, InteractionConfig},
        types::LLMResponse,
    },
};

pub struct LlmClient {
    model: String,
    client: GeminiSDK,
}

impl LlmClient {
    pub fn new(base_url: String, api_key: String, model: String) -> Self {
        let client = GeminiSDK::new(base_url, api_key);
        Self { model, client }
    }

    pub fn get_prompt(
        &self,
        query: &str,
        history: Option<&[ChatMessage]>,
        code_context: Option<&str>,
    ) -> String {
        let history_section = if let Some(history) = history {
            let mut s = String::new();

            for h in history.iter() {
                let string: String = h.into();
                s.push_str(&string);
            }

            s
        } else {
            String::new()
        };

        let mut code_section = String::new();

        if let Some(code) = code_context {
            if !code.is_empty() {
                code_section = format!("### Code Context\n{}\n\n", code);
            }
        }
        format!(
            "You are a codebase assistant. Answer the user's question using the provided context and conversation history.\n\n### Instructions\n- Answer based on the provided code context.\n- Mention relevant file paths.\n- If the context does not contain enough information, state that clearly.\n- You have access to tools. If a tool can help you answer the user's question or perform a requested action, call the appropriate tool.\n\n{code_section}{history_section}### Current Question\n{query}"
        )
    }

    pub async fn get_response(
        &self,
        config: InteractionConfig<'_>,
    ) -> Result<LLMResponse, LLMError> {
        let response = self.client.create_interaction(config).await?;

        Ok(response)
    }

    pub async fn get_response_stream(
        &self,
        config: InteractionConfig<'_>,
    ) -> Result<impl Stream<Item = Result<String, LLMError>>, LLMError> {
        let response = self.client.create_interaction_stream(config).await?;

        Ok(response)
    }
}
