use futures::Stream;
use serde_json::Value;

use crate::core::{
    conversations::ChatMessage,
    llm::{
        ApiKeys, LLMError, LLMResponse,
        gemini_sdk::{GeminiSDK, InteractionConfig},
    },
};

#[derive(Clone, Debug)]
pub struct LlmConfig<'a> {
    pub input: &'a str,
    pub model: Option<&'a str>,
    pub tools: Option<&'a Value>,
    pub response_format: Option<&'a Value>,
    pub history: Option<&'a [ChatMessage<'a>]>,
    pub is_first_call: bool,
}

pub struct LlmClient {
    model: String,
    // client: GeminiSDK,
    api_keys_provider: ApiKeys,
}

impl LlmClient {
    pub fn new(api_key: String, model: String) -> Self {
        // let client = GeminiSDK::new(api_key);
        let api_keys_provider = ApiKeys::new();

        Self {
            model,
            api_keys_provider,
        }
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

    pub async fn get_response(&mut self, config: LlmConfig<'_>) -> Result<LLMResponse, LLMError> {
        let prompt = self.get_prompt(config.input, config.history, None);
        let inteaction_config = InteractionConfig {
            model: None,
            tools: config.tools,
            response_format: config.response_format,
            input: &prompt,
        };

        // let response = self.client.create_interaction(inteaction_config).await?;

        let api_key = self.api_keys_provider.get_key().unwrap();
        let client = GeminiSDK::new(api_key);
        let response = client.create_interaction(inteaction_config).await?;

        Ok(response)
    }

    pub async fn get_response_stream(
        &mut self,
        config: LlmConfig<'_>,
    ) -> Result<impl Stream<Item = Result<String, LLMError>>, LLMError> {
        let prompt = self.get_prompt(config.input, config.history, None);

        let inteaction_config = InteractionConfig {
            model: None,
            tools: config.tools,
            response_format: config.response_format,
            input: &prompt,
        };

        // let response = self
        //     .client
        //     .create_interaction_stream(inteaction_config)
        //     .await?;

        let api_key = self.api_keys_provider.get_key().unwrap();
        let client = GeminiSDK::new(api_key);
        let response = client.create_interaction_stream(inteaction_config).await?;

        Ok(response)
    }
}
