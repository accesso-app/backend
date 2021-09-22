use accesso_core::models;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub(crate) struct Client {
    pub(crate) id: uuid::Uuid,
    // If client is marked as "for developers", some checks will be skipped
    pub(crate) is_dev: bool,
    pub(crate) redirect_uri: Vec<String>,
    pub(crate) secret_key: String,
    pub(crate) title: String,
    pub(crate) allowed_registrations: bool,
}

impl Into<models::Application> for Client {
    fn into(self) -> models::Application {
        models::Application {
            id: self.id,
            is_dev: self.is_dev,
            redirect_uri: self.redirect_uri,
            secret_key: self.secret_key,
            title: self.title,
            allowed_registrations: self.allowed_registrations,
        }
    }
}
