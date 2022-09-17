use crate::domain::HelloError;


#[derive(thiserror::Error, Debug)]
#[error("Something went wrong.")]
pub struct DatabaseError {
    #[from]
    source: anyhow::Error,
}

impl From<HelloError> for DatabaseError {
    fn from(e: HelloError) -> Self {
        match e {
            HelloError::NotFound { source, .. } => source,
            HelloError::DatabaseError(e) => e,
        }
    }
}