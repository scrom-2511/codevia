use std::collections::HashMap;

use crate::ui::{Component, handlers::component_focus::ComponentFocus};

pub struct Scroll {
    components: HashMap<Component, i16>,
}

impl Scroll {
    pub fn new() -> Self {
        let mut components = HashMap::new();

        components.insert(Component::Conversations, 0);
        components.insert(Component::PromptBox, 0);

        Self { components }
    }

    pub fn increment_offset(&mut self, component_focus: &ComponentFocus) -> () {
        let current_focused_component = component_focus.get_current_focused_component();

        if let Some(offset) = self.components.get_mut(&current_focused_component) {
            *offset = offset.saturating_add(1);
        }
    }

    pub fn decrement_offset(&mut self, component_focus: &ComponentFocus) -> () {
        let current_focused_component = component_focus.get_current_focused_component();

        if let Some(offset) = self.components.get_mut(&current_focused_component) {
            *offset = (*offset - 1).max(0);
        }
    }

    pub fn get_offset(&self, component_focus: &ComponentFocus) -> i16 {
        let current_focused_component = component_focus.get_current_focused_component();

        *self
            .components
            .get(&current_focused_component)
            .unwrap_or(&0)
    }
}
