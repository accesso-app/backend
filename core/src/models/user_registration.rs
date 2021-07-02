#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserRegistration {
    pub id: uuid::Uuid,
    pub client_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: uuid::Uuid,
}
