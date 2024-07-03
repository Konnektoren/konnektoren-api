use std::fmt;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    InternalError(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound => write!(f, "Resource not found"),
            RepositoryError::InternalError(err) => write!(f, "Internal error: {}", err),
        }
    }
}

impl std::error::Error for RepositoryError {}