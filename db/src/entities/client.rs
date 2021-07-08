use accesso_core::models;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub(crate) struct Client {
    pub(crate) id: uuid::Uuid,
    pub(crate) redirect_uri: Vec<String>,
    pub(crate) secret_key: String,
    pub(crate) title: String,
    pub(crate) allowed_registrations: bool,
}

impl Into<models::Client> for Client {
    fn into(self) -> models::Client {
        models::Client {
            id: self.id,
            redirect_uri: self.redirect_uri,
            secret_key: self.secret_key,
            title: self.title,
            allowed_registrations: self.allowed_registrations,
        }
    }
}
