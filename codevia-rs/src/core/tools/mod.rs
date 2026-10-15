use crate::core::ErrorKind;

mod discovery;
mod file_system;
mod reader;
mod writer;

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceErrors {
    #[error("Access denied: '{0}' is outside the workspace.")]
    AccessDenied(String),
    #[error("Path not found: '{0}'")]
    NotFound(String),
    #[error("File already exists: '{0}'")]
    AlreadyExists(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Arguments error: {0}")]
    Arguments(String),
}

impl WorkspaceErrors {
    pub fn new(message: impl Into<String>, kind: ErrorKind) -> Self {
        let message = message.into();

        match kind {
            ErrorKind::Arguments => Self::Arguments(message),
            ErrorKind::NotFound => Self::NotFound(message),
            ErrorKind::AccessDenied => Self::AccessDenied(message),
            ErrorKind::AlreadyExists => Self::AlreadyExists(message),
        }
    }
}

impl Into<String> for WorkspaceErrors {
    fn into(self) -> String {
        self.to_string()
    }
}
