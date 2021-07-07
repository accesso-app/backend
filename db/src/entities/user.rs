use accesso_core::models;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub(crate) struct User {
    pub(crate) id: uuid::Uuid,
    pub(crate) email: String,
    pub(crate) password_hash: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) canonical_email: String,
}

impl Into<models::User> for User {
    fn into(self) -> models::User {
        // We need this because we strip NULL chars from the string before sending to database,
        // thus to verify correctly we need to pad with zeroes
        let mut padded = [0u8; 128];
        self.password_hash
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, val)| {
                padded[i] = *val;
            });

        models::User {
            id: self.id,
            email: self.email,
            canonical_email: self.canonical_email,
            password_hash: String::from_utf8(padded.to_vec()).unwrap(),
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
}
