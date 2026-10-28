const PROMPT_PREFIX: &str = "❯ ";

pub struct PromptInputState {
    pub input: String,
}

impl PromptInputState {
    pub fn new() -> Self {
        Self {
            input: PROMPT_PREFIX.to_string(),
        }
    }

    pub fn append(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop(&mut self) {
        if self.input.len() > PROMPT_PREFIX.len() {
            self.input.pop();
        }
    }

    pub fn clear(&mut self) {
        self.input = PROMPT_PREFIX.to_string();
    }
}
