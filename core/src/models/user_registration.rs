use chrono::Utc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserRegistration {
    pub id: uuid::Uuid,
    pub client_id: uuid::Uuid,
    // User registration does not expires!
    pub created_at: chrono::DateTime<Utc>,
    pub user_id: uuid::Uuid,
}
