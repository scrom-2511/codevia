pub mod components;
pub mod handlers;
pub mod styles;

pub struct ComponentConfig {
    pub scrollable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Component {
    Header,
    Conversations,
    Input,
}
