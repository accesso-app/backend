use crate::generated::{
    components::{request_bodies, responses},
    paths::oauth_token::{Error, Response},
};
use actix_web::web;

use responses::{
    OAuthAccessTokenCreated as Created, OAuthAccessTokenFailure as Failure,
    OAuthAccessTokenFailureError as FailureError,
};

use accesso_core::app::oauth::exchange::{
    ExchangeAccessTokenForm, ExchangeFailed, GrantType, OAuthExchange, TokenType,
};

pub async fn route(
    body: web::Json<request_bodies::OAuthAccessTokenExchange>,
    app: web::Data<accesso_app::App>,
) -> Result<Response, Error> {
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

    let created = app
        .oauth_exchange_access_token(form)
        .await
        .map_err(map_exchange_failed)?;

    Ok(Response::Created(Created {
        access_token: created.access_token,
        expires_in: created.expires_in.timestamp(),
        token_type: match created.token_type {
            TokenType::Bearer => responses::OAuthAccessTokenCreatedTokenType::Bearer,
        },
    }))
}

fn map_exchange_failed(error: ExchangeFailed) -> Error {
    use ExchangeFailed::{
        InvalidClient, InvalidGrant, InvalidRequest, InvalidScope, UnauthorizedClient, Unexpected,
    };

    match error {
        Unexpected(e) => Error::InternalServerError(e),
        UnauthorizedClient => Failure {
            error: FailureError::UnauthorizedClient,
        }
        .into(),
        InvalidRequest(e) => Failure { error: e.into() }.into(),
        InvalidClient => Failure {
            error: FailureError::InvalidClient,
        }
        .into(),
        InvalidGrant => Failure {
            error: FailureError::InvalidGrant,
        }
        .into(),
        InvalidScope => Failure {
            error: FailureError::InvalidScope,
        }
        .into(),
    }
}
