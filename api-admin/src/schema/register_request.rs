use accesso_app::Service;
use accesso_core::contracts::{Repository, SecureGenerator};
use async_graphql::{Context, Object, SimpleObject};

#[derive(SimpleObject)]
pub struct RegisterRequest {
    email: String,
    code: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

impl From<accesso_core::models::RegisterRequest> for RegisterRequest {
    fn from(register_request: accesso_core::models::RegisterRequest) -> Self {
        Self {
            email: register_request.email,
            code: register_request.code,
            expires_at: register_request.expires_at,
        }
    }
}

#[derive(Default)]
pub struct QueryRequesterRequest;

#[Object]
impl QueryRequesterRequest {
    pub async fn register_requests(
        &self,
        context: &Context<'_>,
    ) -> async_graphql::Result<Vec<RegisterRequest>> {
        let db = context.data::<Service<dyn Repository>>()?;
        let list = db.register_request_list().await?;
        Ok(list.into_iter().map(Into::into).collect())
    }

    pub async fn register_requests_by_email(
        &self,
        context: &Context<'_>,
        #[graphql(validator(email))] email: String,
        #[graphql(default = 100)] count: u16,
    ) -> async_graphql::Result<Vec<RegisterRequest>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db
            .register_requests_get_by_email(email, count)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub async fn register_requests_search(
        &self,
        context: &Context<'_>,
        query: String,
        #[graphql(default = 100)] count: u16,
    ) -> async_graphql::Result<Vec<RegisterRequest>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db
            .register_requests_search(query, count)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

#[derive(Default)]
pub struct MutationRegisterRequest;

#[Object]
impl MutationRegisterRequest {
    pub async fn register_request_create(
        &self,
        context: &Context<'_>,
        #[graphql(validator(email))] email: String,
    ) -> async_graphql::Result<RegisterRequest> {
        let db = context.data::<Service<dyn Repository>>()?;
        let generator = context.data::<Service<dyn SecureGenerator>>()?;
        let code = generator.confirmation_code();
        let request = accesso_core::models::RegisterRequest::new(email, code);
        let result = db.register_request_save(request).await?;
        Ok(result.into())
    }

    pub async fn register_request_delete_all_for_email(
        &self,
        context: &Context<'_>,
        #[graphql(validator(email))] email: String,
    ) -> async_graphql::Result<u64> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db.register_requests_delete_all_for_email(email).await?)
    }

    pub async fn register_request_delete(
        &self,
        context: &Context<'_>,
        code: String,
    ) -> async_graphql::Result<Option<RegisterRequest>> {
        let db = context.data::<Service<dyn Repository>>()?;
        Ok(db
            .register_request_delete_by_code(code)
            .await?
            .map(Into::into))
    }
}
