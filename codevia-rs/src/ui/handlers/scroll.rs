use std::collections::HashMap;

use crate::ui::{Component, handlers::component_focus::ComponentFocus};

pub struct Scroll {
    components: HashMap<Component, u16>,
    component_focus: ComponentFocus,
}

impl Scroll {
    pub fn new(component_focus: ComponentFocus) -> Self {
        let mut components = HashMap::new();

        components.insert(Component::Conversations, 0);
        components.insert(Component::Input, 0);

        Self {
            components,
            component_focus,
        }
    }

    pub fn increment_offset(&mut self) -> () {
        let current_focused_component = self.component_focus.get_current_focused_component();

        let current_focused_component_scroll_offset =
            self.components.get(&current_focused_component).unwrap();

        if current_focused_component.is_scrollable() {
            self.components.insert(
                current_focused_component,
                current_focused_component_scroll_offset + 1,
            );
        }
    }

    pub fn decrement_offset(&mut self) -> () {
        let current_focused_component = self.component_focus.get_current_focused_component();

        let current_focused_component_scroll_offset =
            self.components.get(&current_focused_component).unwrap();

        if current_focused_component.is_scrollable() {
            self.components.insert(
                current_focused_component,
                current_focused_component_scroll_offset - 1,
            );
        }
    }
}
