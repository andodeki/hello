use crate::domain::hello::errors::HelloError;
use crate::domain::hello::Hello;
// use std::collections::HashSet;
use uuid::Uuid;

pub trait Repository {
    fn hello(&self, hello_id: Uuid) -> Result<Hello, HelloError>;
}
