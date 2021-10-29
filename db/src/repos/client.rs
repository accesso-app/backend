use accesso_core::contracts::{
    ApplicationCreateError, ApplicationForm, ApplicationRepo, UnexpectedDatabaseError,
};
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

    async fn application_create(
        &self,
        application: ApplicationForm,
    ) -> Result<models::Application, ApplicationCreateError> {
        Ok(sqlx::query_as!(
            entities::Client,
            // language=PostgreSQL
            r#"
            INSERT INTO clients (is_dev, redirect_uri, title, secret_key, allowed_registrations)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, is_dev, redirect_uri, title, secret_key, allowed_registrations
            "#,
            application.is_dev,
            &application.redirect_uri,
            application.title,
            application.secret_key,
            application.allowed_registrations,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|error| ApplicationCreateError::Unexpected(error.into()))?
        .into())
    }

    async fn application_edit(
        &self,
        id: uuid::Uuid,
        form: ApplicationForm,
    ) -> Result<Option<models::Application>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            entities::Client,
            // language=PostgreSQL
            r#"
            UPDATE clients SET (is_dev, redirect_uri, title, secret_key, allowed_registrations)
            = ($1, $2, $3, $4, $5)
            WHERE id = $6
            RETURNING id, is_dev, redirect_uri, title, secret_key, allowed_registrations
            "#,
            form.is_dev,
            &form.redirect_uri,
            form.title,
            form.secret_key,
            form.allowed_registrations,
            id,
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|client| client.into())
        .into())
    }
}
