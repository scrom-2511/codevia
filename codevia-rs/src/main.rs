use std::{
    fs::read_to_string,
    io::{self, Error},
};

use serde_json::Value;

use crate::{
    core::{
        ChatHistoryParser, ChatMessage, Conversations, FileSystem, LlmClient, LlmConfig, Redis,
        ToolRegistry,
    },
    ui::handlers::{component_focus::ComponentFocus, scroll::Scroll},
};

mod core;
mod ui;

pub struct State {
    component_focus: ComponentFocus,
    scroll: Scroll,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let component_focus = ComponentFocus::new();
    // let scroll = Scroll::new(component_focus.clone());

    // let mut state = State {
    //     component_focus,
    //     scroll,
    // };

    // ratatui::run(|terminal| {
    //     loop {
    //         terminal.draw(|frame| {
    //             let main_layout = Layout::default()
    //                 .direction(Direction::Vertical)
    //                 .constraints([
    //                     Constraint::Length(3),
    //                     Constraint::Min(1),
    //                     Constraint::Length(3),
    //                 ])
    //                 .split(frame.area());

    //             render_header(main_layout[0], frame, &state);
    //             render_body(main_layout[1], frame);
    //         })?;

    //         let continue_loop = keyboard_input_handler(&mut state)?;
    //         if !continue_loop {
    //             break;
    //         }
    //     }
    //     Ok(())
    // })

    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY_1").unwrap();

    let mut llm = LlmClient::new(api_key, String::from("gemini-3.5-flash-lite"));

    let redis = Redis::new().unwrap();

    let conversations = Conversations::new(redis);
    let conversation_id = conversations.create_conversation();

    let tools = read_to_string("./src/core/tools/tools.json").unwrap();

    let tools_value: Value = serde_json::from_str(&tools).unwrap();

    println!("{} ", "after tools value");

    let fs = FileSystem::new().unwrap();
    let tool_registry = ToolRegistry::new(&fs);

    loop {
        println!("inside 1st loop");
        let mut user_prompt = String::new();

        let bytes_read = io::stdin().read_line(&mut user_prompt).unwrap();
        if bytes_read == 0 {
            break;
        }

        let user_prompt = user_prompt.trim().to_string();
        if user_prompt.is_empty() {
            continue;
        }

        let _ = conversations
            .add_message(&conversation_id, &ChatMessage::user(&user_prompt))
            .await;

        let mut latest_fn_result: Option<String> = None;

        loop {
            let input = if let Some(latest_result) = &latest_fn_result {
                &format!("the tool call result is: {}", latest_result)
            } else {
                &user_prompt
            };

            let raw_history = conversations
                .get_raw_history(&conversation_id)
                .await
                .unwrap();

            let parsed_history = raw_history.parse_chat_history().unwrap();
            let history = parsed_history.as_slice();

            let response = llm
                .get_response(LlmConfig {
                    input,
                    model: None,
                    tools: Some(&tools_value),
                    response_format: None,
                    history: Some(&history),
                    is_first_call: latest_fn_result.is_none(),
                })
                .await
                .unwrap();

            let response = response.response;

            let mut has_tool_call = false;

            if let Some(steps) = response["steps"].as_array() {
                println!("number of steps: {}", steps.len());
                for step in steps {
                    println!("step: {}", step);
                    match step["type"].as_str() {
                        Some("thought") => {
                            // Ignore this step and continue to the next one.
                            continue;
                        }
                        Some("function_call") => {
                            has_tool_call = true;

                            let tool_name = step["name"].as_str().unwrap();
                            let tool_args = &step["arguments"];
                            println!("tool_name: {}", tool_name);
                            println!("tool_args: {}", tool_args);

                            let _ = conversations.add_message(
                                &conversation_id,
                                &ChatMessage::model(format!(
                                    "tool call: name -> {}, arguments -> {}",
                                    tool_name, tool_args
                                )),
                            );

                            match tool_registry.execute(tool_name, tool_args) {
                                Ok(output) => {
                                    println!("tool output:\n{output}");
                                    latest_fn_result = Some(output);

                                    let _ = conversations
                                        .add_message(
                                            &conversation_id,
                                            &ChatMessage::user(format!(
                                                "The output of {} is :\n {}",
                                                tool_name,
                                                latest_fn_result.as_ref().unwrap()
                                            )),
                                        )
                                        .await;
                                }
                                Err(err) => {
                                    eprintln!("tool error: {err}");
                                    latest_fn_result = Some(format!("tool error: {err}"));
                                }
                            }
                        }
                        Some("model_output") => {
                            let output = step["content"][0]["text"]
                                .as_str()
                                .unwrap_or("No text content returned");

                            println!("model_output: {}", output);

                            let _ = conversations
                                .add_message(&conversation_id, &ChatMessage::model(output))
                                .await;
                        }
                        _ => {}
                    }
                }
            }

            if !has_tool_call {
                break;
            }
        }
    }

    // let string = r#"{"created":"2026-10-26T10:43:52Z","id":"v1_Chc0aTdmYW96VkUtTEY2OTRQbnFhOGtBWRIXNGk3ZmFvelZFLUxGNjk0UG5xYThrQVk","model":"gemini-3.6-flash","object":"interaction","service_tier":"standard","status":"requires_action","steps":[{"signature":"Es8CCswCAWkUfRMbSQgzYKlFqdxs91QwkZTTtYrPXZ8Tzyqka/Vd2U5WslUug5YoIyIiqEO5fHSD6TlxmVIgcoAB6CBHmFbjYywNgttfuGRs+nGR3MGw8XEe/8EmdGnCWGeVslmIiMyxXbpSyglMj5ne8eXg358dX8aJ0uc/GKHwl1VyUxPJ1Vgd9dDSu9nvYTh/ADhHmJhozhqWw1Kp+HBMuvuujNq+hbwxKFLZtwjHLg8sF8J6xkBSkZdihdAOOC0z2LsOJomKLoUhPXE6uLuMY99f7naFqHO+jscwizg6eekbKOQC5z9GzIKwmAtbgpWPG5GGIzn10few6bsiZpFHxaa2zjMHNN8IFlNjwdMbNG1uXvvd3/F2v6h1WVYJvcXnBvaaqiNvpHwP3K8uR0Jf5punXGdNZaONCbU84FIZxm/P2Hq+xo8ZOEzu3NAqXqA=","type":"thought"},{"arguments":{"path":".","recursive":true},"id":"call_3656597","name":"list_files","type":"function_call"}],"updated":"2026-10-26T10:43:52Z","usage":{"input_tokens_by_modality":[{"modality":"text","tokens":2380}],"total_cached_tokens":0,"total_input_tokens":2380,"total_output_tokens":20,"total_thought_tokens":51,"total_tokens":2451,"total_tool_use_tokens":0}}"#;

    // let value = serde_json::from_str::<Value>(string).unwrap();

    // let string = serde_json::to_string_pretty(&value).unwrap();

    // println!("{}", string);

    Ok(())
}
