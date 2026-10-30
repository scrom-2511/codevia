#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolType {
    Read,
    Write,
    Search,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role<'a> {
    Model,
    User,
    ToolCall(ToolType, Vec<&'a str>),
}

#[derive(Debug, Clone)]
pub struct ChatMessage<'a> {
    pub role: Role<'a>,
    pub message: &'a str,
}

#[derive(Debug, Clone)]
pub struct ConversationState<'a> {
    pub conversation: Vec<ChatMessage<'a>>,
}

impl<'a> ConversationState<'a> {
    pub fn new() -> Self {
        Self {
            conversation: Vec::new(),
        }
    }

    pub fn add_message(&mut self, role: Role<'a>, message: &'a str) {
        let chat_message = ChatMessage { role, message };
        self.conversation.push(chat_message);
    }

    pub fn clear(&mut self) {
        self.conversation.clear();
    }
}
