use accesso_core::contracts::repo::UserRegistrationsRepo;
use accesso_core::contracts::{UnexpectedDatabaseError, UserRegistrationCreateError};
use accesso_core::models;
use accesso_core::models::{Application, User};
use sqlx::types::Uuid;

use crate::entities::UserRegistration;
use crate::mappers::sqlx_error_to_user_registration_error;
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
        .await?
        .map(Into::into))
    }

    async fn user_registration_find_for_client(
        &self,
        client: &Application,
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
        .await?
        .map(Into::into))
    }

    async fn user_registration_create(
        &self,
        client: &Application,
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

    async fn user_registration_list_for_client(
        &self,
        application_id: uuid::Uuid,
    ) -> Result<Vec<models::UserRegistration>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            UserRegistration,
            // language=PostgreSQL
            r#"
            SELECT regs.*
            FROM user_registrations regs
            WHERE regs.client_id = $1
            "#,
            application_id,
        )
        .fetch_all(&self.pool)
        .await
        .map(|list| list.into_iter().map(Into::into).collect())?)
    }

    async fn user_registration_list_for_user(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Vec<models::UserRegistration>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            UserRegistration,
            // language=PostgreSQL
            r#"
            SELECT user_registrations.*
            FROM user_registrations
            WHERE user_registrations.user_id = $1
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .map(|list| list.into_iter().map(Into::into).collect())?)
    }

    async fn user_access_tokens_count(
        &self,
        user_id: Uuid,
    ) -> Result<u64, UnexpectedDatabaseError> {
        Ok(sqlx::query_scalar!(
            // language=PostgreSQL
            r#"
            SELECT COUNT(access_tokens.*)
            FROM access_tokens
            INNER JOIN user_registrations
                ON user_registrations.id = access_tokens.registration_id
            WHERE user_registrations.user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or_default() as u64)
    }
}
