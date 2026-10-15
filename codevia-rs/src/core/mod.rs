mod tools;

pub enum ErrorKind {
    Arguments,
    NotFound,
    AccessDenied,
    AlreadyExists,
}
