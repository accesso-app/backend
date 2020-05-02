#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Client {
    pub id: uuid::Uuid,
    pub redirect_uri: Vec<String>,
    pub scopes: Vec<String>,
    pub title: String,
}
