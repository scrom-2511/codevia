mod conversations;
mod db;
mod llm;
mod tools;

pub enum ErrorKind {
    Arguments,
    NotFound,
    AccessDenied,
    AlreadyExists,
}
