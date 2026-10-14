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
            component_focus: component_focus,
        }
    }
}
