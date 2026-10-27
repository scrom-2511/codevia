#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ApiKeys {
    pub keys: Vec<String>,
    pub current_index: usize,
}

impl ApiKeys {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        let mut keys = Vec::new();
        for i in 1..=10 {
            let key_name = format!("API_KEY_{}", i);

            if let Ok(key) = std::env::var(&key_name) {
                let trimmed = key.trim();
                if !trimmed.is_empty() {
                    keys.push(trimmed.to_string());
                }
            }
        }

        Self {
            keys,
            current_index: 0,
        }
    }

    pub fn from_keys(keys: Vec<String>) -> Self {
        Self {
            keys,
            current_index: 0,
        }
    }

    pub fn get_key(&mut self) -> Option<String> {
        if self.keys.is_empty() {
            return None;
        }

        if self.current_index >= self.keys.len() {
            self.current_index = 0;
        }

        let key = self.keys[self.current_index].clone();
        self.current_index += 1;

        if self.current_index >= self.keys.len() {
            self.current_index = 0;
        }

        Some(key)
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}
