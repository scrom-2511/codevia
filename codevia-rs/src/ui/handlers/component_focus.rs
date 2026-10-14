use strum::IntoEnumIterator;

use crate::ui::Component;

#[derive(Clone)]
pub struct ComponentFocus {
    focus: Component,
    components: Vec<Component>,
    current_component_index: usize,
}

impl ComponentFocus {
    pub fn new() -> Self {
        let components: Vec<Component> = Component::iter().collect();
        let current_component_index = 0;
        let focus = components[current_component_index];

        Self {
            components,
            current_component_index,
            focus,
        }
    }

    pub fn change_focus(&mut self) -> () {
        if self.current_component_index == self.components.len() - 1 {
            self.current_component_index = 0;
        } else {
            self.current_component_index += 1;
        }

        self.focus = self.components[self.current_component_index];
    }

    pub fn get_current_focused_component(&self) -> Component {
        self.focus
    }
}
