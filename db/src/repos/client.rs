use accesso_core::contracts::{ClientRepo, UnexpectedDatabaseError};
use accesso_core::models;

use crate::entities::Client;
use crate::mappers::sqlx_error_to_unexpected;
use crate::Database;

#[async_trait]
impl ClientRepo for Database {
    async fn client_find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<models::Client>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            Client,
            // language=PostgreSQL
            r#"
            SELECT id,
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
        .await
        .map_err(sqlx_error_to_unexpected)?
        .map(Into::into))
    }
}

#[cfg(feature = "testing")]
#[async_trait]
impl ClientRepo for accesso_core::contracts::MockDb {
    async fn client_find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<Client>, UnexpectedDatabaseError> {
        self.client.client_find_by_id(id).await
    }
}
