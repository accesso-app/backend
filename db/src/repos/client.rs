use accesso_core::contracts::{ApplicationRepo, UnexpectedDatabaseError};
use accesso_core::models;

use crate::entities;
use crate::Database;

#[async_trait]
impl ApplicationRepo for Database {
    async fn application_find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<models::Application>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            entities::Client,
            // TODO: rename table to `applications`
            // language=PostgreSQL
            r#"
            SELECT id,
                   is_dev,
                   redirect_uri,
                   secret_key,
                   title,
                   allowed_registrations
            FROM clients
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn application_list(&self) -> Result<Vec<models::Application>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            entities::Client,
            // language=PostgreSQL
            r#"
            SELECT id, is_dev, redirect_uri, secret_key, title, allowed_registrations
            FROM clients
            "#,
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|client| client.into())
        .collect())
    }
}
