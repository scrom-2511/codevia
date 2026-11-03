use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::ui::{Component, handlers::component_focus::ComponentFocus};

pub struct Scroll {
    components: HashMap<Component, (i16, i16, bool)>,
}

impl Scroll {
    pub fn new() -> Self {
        let mut components = HashMap::new();

        for component in Component::iter() {
            components.insert(component, (0, 0, true));
        }

        Self { components }
    }

    pub fn increment_offset(&mut self, component_focus: &ComponentFocus) -> () {
        let current_focused_component = component_focus.get_current_focused_component();

        if !current_focused_component.is_scrollable() {
            return;
        }

        if let Some((offset, max, follow_bottom)) =
            self.components.get_mut(&current_focused_component)
        {
            if *offset < *max {
                *offset = offset.saturating_add(1);
            }

            if *offset >= *max {
                *follow_bottom = true
            }
        }
    }

    pub fn decrement_offset(&mut self, component_focus: &ComponentFocus) -> () {
        let current_focused_component = component_focus.get_current_focused_component();

        if !current_focused_component.is_scrollable() {
            return;
        }

        if let Some((offset, _, follow_bottom)) =
            self.components.get_mut(&current_focused_component)
        {
            *offset = offset.saturating_sub(1);
            *follow_bottom = false;
        }
    }

    pub fn scroll_to_bottom(&mut self, component: &Component) {
        if let Some((offset, max, follow_bottom)) = self.components.get_mut(component) {
            *offset = *max;
            *follow_bottom = true;
        }
    }

    pub fn get_offset(&self, component: &Component) -> i16 {
        self.components
            .get(component)
            .map(|(offset, ..)| *offset)
            .unwrap_or(0)
    }

    pub fn set_max_offset(&mut self, component: &Component, max_offset: i16) {
        let max_offset = max_offset.max(0);

        if let Some((offset, max, follow_bottom)) = self.components.get_mut(component) {
            *max = max_offset;

            if *follow_bottom {
                *offset = max_offset;
            } else {
                *offset = (*offset).min(max_offset);
            }
        }
    }
}
