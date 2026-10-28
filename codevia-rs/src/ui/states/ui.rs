use crate::ui::{component_focus::ComponentFocus, scroll::Scroll};

pub struct UiState<'a> {
    pub component_focus: &'a mut ComponentFocus,
    pub scroll: Scroll,
}

impl<'a> UiState<'a> {
    pub fn new(component_focus: &'a mut ComponentFocus, scroll: Scroll) -> Self {
        Self {
            component_focus,
            scroll,
        }
    }
}
