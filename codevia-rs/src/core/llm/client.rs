use futures::Stream;
use serde_json::Value;

use crate::{
    core::llm::{
        ApiKeys, LLMError, LLMResponse,
        gemini_sdk::{GeminiSDK, InteractionConfig},
    },
    shared::ChatMessage,
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
    pub fn new(_api_key: String, model: String) -> Self {
        // let client = GeminiSDK::new(api_key);
        let api_keys_provider = ApiKeys::new();

        Self {
            model,
            api_keys_provider,
        }
    }

    pub fn get_system_instruction(&self, history: Option<&[ChatMessage]>) -> String {
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

        format!(
            r#"You are an intelligent codebase assistant. Your task is to answer the user's questions and perform requested actions using the provided codebase context and conversation history.
            ### Instructions
            1. Use the provided context: Base your answers on the available codebase context and conversation history. Do not make assumptions or invent information.
            2. Reference file paths: Whenever relevant, mention the exact file paths, functions, classes, or components involved.
            3. Be transparent: If the provided context does not contain enough information to answer a question confidently, clearly state what is missing.
            4. Use tools when necessary: You have access to tools. If a tool can help you investigate the codebase, retrieve additional information, or perform the requested action, use the appropriate tool.
            5. Keep tool updates brief: Before or while calling tools, provide a short status message such as "Reading files..." or "Analyzing code...". Avoid lengthy explanations.
            6. No Markdown formatting: Respond using plain text. Do not use Markdown headings, bullet points, tables, code fences, or other Markdown formatting.
            7. Be concise and relevant: Answer the user's question directly. Avoid unnecessary explanations, repetition, or unrelated information.
            8. Preserve conversation context: Consider previous messages when interpreting the current question. Do not ask for information that has already been provided.
            9. Perform requested actions: When the user asks you to make changes, fix bugs, or implement something, use the available tools to complete the task rather than merely explaining how to do it.


            ### Conversation History
            {}"#,
            history_section
        )
    }

    pub async fn get_response(&mut self, config: LlmConfig<'_>) -> Result<LLMResponse, LLMError> {
        let prompt = config.input;
        let history = self.get_system_instruction(config.history);

        let inteaction_config = InteractionConfig {
            model: None,
            tools: config.tools,
            response_format: config.response_format,
            input: &prompt,
            system_instruction: Some(&history.as_str()),
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
        let prompt = config.input;
        let history = self.get_system_instruction(config.history);

        let inteaction_config = InteractionConfig {
            model: None,
            tools: config.tools,
            response_format: config.response_format,
            input: &prompt,
            system_instruction: Some(&history.as_str()),
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
