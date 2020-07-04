use crate::generated::{
    components::{request_bodies, responses},
    paths::oauth_token::Response,
};
use accesso_public_logic::models;
use actix_swagger::Answer;
use actix_web::{dev, web, FromRequest, HttpMessage};

use responses::{
    OAuthAccessTokenCreated as Created, OAuthAccessTokenFailure as Failure,
    OAuthAccessTokenFailureError as FailureError,
};

pub async fn route(
    body: web::Json<request_bodies::OAuthAccessTokenExchange>,
    app: web::Data<crate::App>,
) -> Answer<'static, Response> {
    Response::InternalServerError.answer()
}
