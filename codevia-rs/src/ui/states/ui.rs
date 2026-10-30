use crate::ui::{Component, component_focus::ComponentFocus, scroll::Scroll};

pub struct UiState {
    pub component_focus: ComponentFocus,
    pub scroll: Scroll,
}

impl UiState {
    pub fn new(component_focus: ComponentFocus, scroll: Scroll) -> Self {
        Self {
            component_focus,
            scroll,
        }
    }

    pub fn is_current_component_focused(&self, component: &Component) -> bool {
        self.component_focus.get_current_focused_component() == *component
    }
}
