use strum::EnumIter;

pub mod components;
pub mod handlers;
pub mod styles;

pub struct ComponentConfig {
    pub scrollable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Component {
    Header,
    Conversations,
    Input,
}

impl Component {
    pub fn component(&self) -> ComponentConfig {
        match self {
            Self::Header => ComponentConfig { scrollable: false },
            Self::Conversations => ComponentConfig { scrollable: true },
            Self::Input => ComponentConfig { scrollable: true },
        }
    }

    pub fn is_scrollable(&self) -> bool {
        self.component().scrollable
    }
}
