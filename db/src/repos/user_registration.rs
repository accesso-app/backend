use accesso_core::contracts::repo::UserRegistrationsRepo;
use accesso_core::contracts::{UnexpectedDatabaseError, UserRegistrationCreateError};
use accesso_core::models;
use accesso_core::models::{Client, User};

use crate::entities::UserRegistration;
use crate::mappers::{sqlx_error_to_unexpected, sqlx_error_to_user_registration_error};
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

    async fn user_registration_find_for_client(
        &self,
        client: &Client,
        user: &User,
    ) -> Result<Option<models::UserRegistration>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            UserRegistration,
            // language=PostgreSQL
            r#"
            SELECT regs.*
            FROM user_registrations regs
            WHERE regs.client_id = $1 AND regs.user_id = $2
            "#,
            client.id,
            user.id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_unexpected)?
        .map(Into::into))
    }

    async fn user_registration_create(
        &self,
        client: &Client,
        user: &User,
    ) -> Result<models::UserRegistration, UserRegistrationCreateError> {
        Ok(sqlx::query_as!(
            UserRegistration,
            // language=PostgreSQL
            r#"
                INSERT INTO user_registrations (client_id, user_id)
                VALUES ($1, $2)
                RETURNING user_registrations.*
                "#,
            client.id,
            user.id,
        )
        .fetch_one(&self.pool)
        .await
        .map(Into::into)
        .map_err(sqlx_error_to_user_registration_error)?)
    }
}
