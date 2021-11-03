use async_graphql::{EmptySubscription, MergedObject, Object, Schema, SchemaBuilder};

mod access_token;
mod application;
mod register_request;
mod user;
mod user_registration;

#[derive(MergedObject, Default)]
pub struct Query(
    CommonQuery,
    access_token::QueryAccessToken,
    application::QueryApplication,
    register_request::QueryRequesterRequest,
    user::QueryUser,
);

#[derive(Default)]
struct CommonQuery;

#[Object]
impl CommonQuery {
    async fn version(&self) -> &'static str {
        "0.1"
    }
}

#[derive(MergedObject, Default)]
pub struct Mutation(
    application::MutationApplication,
    register_request::MutationRegisterRequest,
    user::MutationUser,
);

pub type AdminSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).limit_depth(8)
}
