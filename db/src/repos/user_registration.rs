use accesso_core::contracts::repo::UserRegistrationsRepo;
use accesso_core::contracts::UnexpectedDatabaseError;
use accesso_core::models;

use crate::entities::UserRegistration;
use crate::mappers::sqlx_error_to_unexpected;
use crate::Database;

#[async_trait]
impl UserRegistrationsRepo for Database {
    async fn user_registration_get_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<models::UserRegistration>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            UserRegistration,
            // language=PostgreSQL
            r#"
            SELECT user_registrations.*
                FROM user_registrations
                    WHERE user_registrations.id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_unexpected)?
        .map(Into::into))
    }
}
