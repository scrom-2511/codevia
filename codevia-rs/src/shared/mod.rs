use std::{borrow::Cow, fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::core::ConversationErrors;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub enum ToolType {
    Search,
    Read,
    Write,
    Analyze,
}

impl ToolType {
    pub fn as_str(&self) -> &str {
        match self {
            ToolType::Read => "Read",
            ToolType::Write => "Write",
            ToolType::Search => "Search",
            ToolType::Analyze => "Analyze",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToolCallMetadata<'a> {
    pub category: ToolType,
    pub paths: Vec<&'a str>,
}

impl<'a> ToolCallMetadata<'a> {
    pub fn new(category: ToolType, paths: Vec<&'a str>) -> Self {
        Self { category, paths }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "tool_call")]
    ToolCall(ToolType, Vec<String>),
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Model => "model",
            Self::ToolCall(..) => "tool_call",
        }
    }

    pub fn capitalize(&self) -> &'static str {
        match self {
            Self::User => "USER",
            Self::Model => "MODEL",
            Self::ToolCall(..) => "TOOL_CALL",
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Role {
    type Err = ConversationErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(Self::User),
            "model" => Ok(Self::Model),
            _ => Err(ConversationErrors::InvalidRole(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage<'a> {
    pub role: Role,
    #[serde(borrow)]
    pub content: Cow<'a, str>,
}

impl<'a> ChatMessage<'a> {
    pub fn new(role: Role, content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<Cow<'a, str>>) -> Self {
        Self::new(Role::User, content)
    }

    pub fn model(content: impl Into<Cow<'a, str>>) -> Self {
        Self::new(Role::Model, content)
    }
}

impl<'a> Into<String> for ChatMessage<'a> {
    fn into(self) -> String {
        let mut result = String::new();

        result.push_str(&self.role.capitalize());
        result.push_str(": ");
        result.push_str(&self.content);

        result
    }
}

impl<'a> From<&ChatMessage<'a>> for String {
    fn from(message: &ChatMessage) -> Self {
        let mut result = String::new();

        result.push_str(&message.role.capitalize());
        result.push_str(": ");
        result.push_str(&message.content);

        result
    }
}
