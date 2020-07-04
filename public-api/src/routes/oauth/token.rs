use crate::generated::{
    components::{request_bodies, responses},
    paths::oauth_token::Response,
};
use actix_swagger::Answer;
use actix_web::web;

use responses::{
    OAuthAccessTokenCreated as Created, OAuthAccessTokenFailure as Failure,
    OAuthAccessTokenFailureError as FailureError,
};

pub async fn route(
    body: web::Json<request_bodies::OAuthAccessTokenExchange>,
    app: web::Data<crate::App>,
) -> Answer<'static, Response> {
    use accesso_public_logic::app::oauth::exchange::{
        ExchangeAccessTokenForm,
        ExchangeFailed::{InvalidClient, InvalidScope, Unauthorized, Unexpected},
        GrantType, OAuthExchange, TokenType,
    };

    let grant_type = match body.grant_type {
        request_bodies::OAuthAccessTokenExchangeGrantType::AuthorizationCode => {
            GrantType::AuthorizationCode
        }
    };

    let form = ExchangeAccessTokenForm {
        grant_type,
        code: body.code.clone(),
        redirect_uri: body.redirect_uri.clone(),
        client_id: body.client_id.clone(),
        client_secret: body.client_secret.clone(),
    };

    let mut app = app.write().unwrap();

    match app.oauth_exchange_access_token(form) {
        // FailureError::InvalidRequest?
        Err(InvalidClient) => Response::BadRequest(Failure {
            error: FailureError::InvalidClient,
        }),
        // FailureError::InvalidGrant?
        Err(InvalidScope) => Response::BadRequest(Failure {
            error: FailureError::InvalidScope,
        }),
        Err(Unauthorized) => Response::BadRequest(Failure {
            error: FailureError::UnauthorizedClient,
        }),
        // FailureError::UnsupportedGrantType?
        Err(Unexpected) => Response::InternalServerError,

        Ok(created) => Response::Created(Created {
            access_token: created.access_token,
            expires: created.expires.timestamp(),
            token_type: match created.token_type {
                TokenType::Bearer => responses::OAuthAccessTokenCreatedTokenType::Bearer,
            },
        }),
    }
    .answer()
}
