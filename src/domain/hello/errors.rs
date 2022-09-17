use crate::domain::DatabaseError;
use uuid::Uuid;



#[derive(thiserror::Error, Debug)]
pub enum HelloError {
    #[error("There is no user with id {id:?}.")]
    NotFound {
        id: Uuid,
        source: DatabaseError,
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}


