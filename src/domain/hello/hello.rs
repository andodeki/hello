use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Hello {
    pub id: Uuid,
    pub greet: String,
}