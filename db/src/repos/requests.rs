use accesso_core::contracts::repo::RequestsRepo;
use accesso_core::contracts::{SaveRegisterRequestError, UnexpectedDatabaseError};
use accesso_core::models;
use accesso_core::models::RegisterRequest;

use crate::entities::RegistrationRequest;
use crate::mappers::sqlx_error_to_save_register_request_error;
use crate::Database;

#[async_trait]
impl RequestsRepo for Database {
    async fn register_request_save(
        &self,
        request: models::RegisterRequest,
    ) -> Result<models::RegisterRequest, SaveRegisterRequestError> {
        let request = RegistrationRequest::from(request);

        Ok(sqlx::query_as!(
            RegistrationRequest,
            // language=PostgreSQL
            r#"
            INSERT INTO registration_requests
                (confirmation_code, email, expires_at)
            VALUES ($1, $2, $3)
            RETURNING confirmation_code, email, expires_at
            "#,
            request.confirmation_code,
            request.email,
            request.expires_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_save_register_request_error)
        .map(Into::into)?)
    }

    async fn register_request_get_by_code(
        &self,
        code: String,
    ) -> Result<Option<models::RegisterRequest>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            RegistrationRequest,
            // language=PostgreSQL
            r#"
            SELECT confirmation_code, email, expires_at
            FROM registration_requests
            WHERE confirmation_code = $1
              AND expires_at > $2
            "#,
            code,
            chrono::Utc::now()
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }

    async fn register_requests_get_by_email(
        &self,
        email: String,
        count: u16,
    ) -> Result<Vec<RegisterRequest>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            RegistrationRequest,
            // language=PostgreSQL
            r#"
            SELECT confirmation_code, email, expires_at
            FROM registration_requests
            WHERE email = $1
            LIMIT $2
            "#,
            email,
            count as i16
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
    }

    async fn register_requests_delete_all_for_email(
        &self,
        email: String,
    ) -> Result<u64, UnexpectedDatabaseError> {
        Ok(sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE
            FROM registration_requests
            WHERE email = $1
            "#,
            email
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }

    async fn register_request_list(
        &self,
    ) -> Result<Vec<models::RegisterRequest>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            RegistrationRequest,
            // language=PostgreSQL
            r#"
                SELECT registration_requests.*
                FROM registration_requests
                "#
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
    }

    async fn register_requests_search(
        &self,
        query: String,
        count: u16,
    ) -> Result<Vec<RegisterRequest>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            RegistrationRequest,
            // language=PostgreSQL
            r#"
            SELECT registration_requests.*
            FROM registration_requests
            WHERE registration_requests.email ILIKE $1
                OR registration_requests.confirmation_code ILIKE $1
            LIMIT $2
            "#,
            format!("%{}%", query),
            count as i16,
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
    }

    async fn register_request_delete_by_code(
        &self,
        code: String,
    ) -> Result<Option<RegisterRequest>, UnexpectedDatabaseError> {
        Ok(sqlx::query_as!(
            RegistrationRequest,
            // language=PostgreSQL
            r#"
            DELETE
            FROM registration_requests
            WHERE confirmation_code = $1
            RETURNING registration_requests.*
            "#,
            code
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }
}
