use std::env;

pub struct HeaderState {
    pub project_name: String,
    pub tokens_used: String,
}

impl HeaderState {
    pub fn new() -> Self {
        let project_name = env::current_dir().unwrap().display().to_string();

        Self {
            project_name,
            tokens_used: String::from("0"),
        }
    }

    pub fn update_tokens(&mut self, tokens: u32) -> () {
        let current_tokens = self.tokens_used.parse::<u64>().unwrap_or(0);
        let updated_tokens = current_tokens.saturating_add(u64::from(tokens));

        self.tokens_used = match updated_tokens {
            n if n >= 1_000_000_000_000 => format!("{:.1}T", n as f64 / 1_000_000_000_000.0),
            n if n >= 1_000_000_000 => format!("{:.1}B", n as f64 / 1_000_000_000.0),
            n if n >= 1_000_000 => format!("{:.1}M", n as f64 / 1_000_000.0),
            n if n >= 1_000 => format!("{:.1}K", n as f64 / 1_000.0),
            n => n.to_string(),
        };

        self.tokens_used = self.tokens_used.replace(".0", "");
    }
}
