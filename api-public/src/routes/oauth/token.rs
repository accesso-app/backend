use crate::generated::{
    components::{request_bodies, responses},
    paths::oauth_token::Response,
};
use crate::lib::{blocking, EachResult};
use actix_swagger::Answer;
use actix_web::web;

use responses::{
    OAuthAccessTokenCreated as Created, OAuthAccessTokenFailure as Failure,
    OAuthAccessTokenFailureError as FailureError,
};

use accesso_core::app::oauth::exchange::{
    ExchangeAccessTokenForm,
    ExchangeFailed::{
        InvalidClient, InvalidGrant, InvalidRequest, InvalidScope, UnauthorizedClient, Unexpected,
    },
    GrantType, OAuthExchange, TokenType,
};

pub async fn route(
    body: web::Json<request_bodies::OAuthAccessTokenExchange>,
    app: web::Data<accesso_app::App>,
) -> Answer<'static, Response> {
    let grant_type = match body.grant_type {
        request_bodies::OAuthAccessTokenExchangeGrantType::AuthorizationCode => {
            GrantType::AuthorizationCode
        }
    };

    let form = ExchangeAccessTokenForm {
        grant_type,
        code: body.code.clone(),
        redirect_uri: body.redirect_uri.clone(),
        client_id: body.client_id,
        client_secret: body.client_secret.clone(),
    };

    blocking(Response::InternalServerError.answer(), move || {
        match app.oauth_exchange_access_token(form) {
            Err(InvalidRequest) => Response::BadRequest(Failure {
                error: FailureError::InvalidRequest,
            }),
            Err(InvalidClient) => Response::BadRequest(Failure {
                error: FailureError::InvalidClient,
            }),
            Err(InvalidGrant) => Response::BadRequest(Failure {
                error: FailureError::InvalidGrant,
            }),
            Err(InvalidScope) => Response::BadRequest(Failure {
                error: FailureError::InvalidScope,
            }),
            Err(UnauthorizedClient) => Response::BadRequest(Failure {
                error: FailureError::UnauthorizedClient,
            }),
            // FailureError::UnsupportedGrantType?
            Err(Unexpected) => Response::InternalServerError,

            Ok(created) => Response::Created(Created {
                access_token: created.access_token,
                expires_in: created.expires_in.timestamp(),
                token_type: match created.token_type {
                    TokenType::Bearer => responses::OAuthAccessTokenCreatedTokenType::Bearer,
                },
            }),
        }
        .answer()
    })
    .await
    .get()
}
