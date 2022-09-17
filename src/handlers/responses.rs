use crate::domain::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HelloResponse {
    pub hello: Hello,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Hello {
    pub id: Uuid,
    pub greet: String,
}

impl From<hello::Hello> for HelloResponse {
    fn from(p: hello::Hello) -> Self {
        Self {
            hello: Hello {
                id: p.id,
                greet: p.greet,
            },
        }
    }
}