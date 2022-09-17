use crate::db::Repo;
use crate::domain::hello::{Hello, HelloError};

use uuid::Uuid;

#[derive(Clone)]
pub struct Repository(pub Repo);

impl crate::domain::repositories::Repository for Repository {
    fn hello(&self, hello_id: Uuid) -> Result<Hello, HelloError>{
        let my_uuid = Uuid::new_v4();
        
        let greet = Hello {
            id: my_uuid,
            greet: "Hello, World!".to_owned(),
            // greet: todo!(),
        };
        Ok(Hello::from(greet))
    }
}