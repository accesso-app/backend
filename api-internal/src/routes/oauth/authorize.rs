use crate::generated::{
    components::{request_bodies, responses},
    paths::oauth_authorize_request::{Error, Response},
};
use actix_web::web;

use crate::session::Session;
use accesso_core::app::oauth::authorize::RequestAuthCodeFailed;
use responses::{
    OAuthAuthorizeDone as Success, OAuthAuthorizeRequestFailure as Failure,
    OAuthAuthorizeRequestFailureError as FailureVariant,
};

pub async fn route(
    auth: Session,
    body: web::Json<request_bodies::OAuthAuthorize>,
    app: web::Data<accesso_app::App>,
) -> Result<Response, Error> {
    use accesso_core::app::oauth::authorize::{OAuthAuthorize, RequestAuthCode};

    let form = RequestAuthCode {
        response_type: match body.response_type {
            request_bodies::OAuthAuthorizeResponseType::Code => "code".to_owned(),
        },
        client_id: body.client_id,
        redirect_uri: body.redirect_uri.clone(),
        scopes: body.scope.clone().map_or(vec![], |scope| {
            scope.split(' ').map(ToOwned::to_owned).collect()
        }),
        state: body.state.clone(),
    };

    let created = app
        .oauth_request_authorize_code(Some(auth.user), form)
        .await
        .map_err(map_request_auth_code_error)?;

    Ok(Response::Ok(Success {
        redirect_uri: created.redirect_uri,
        code: created.code,
        state: created.state,
    }))
}

fn map_request_auth_code_error(error: RequestAuthCodeFailed) -> Error {
    use RequestAuthCodeFailed::{
        AccessDenied, InvalidRequest, InvalidScope, ServerError, TemporarilyUnavailable,
        Unauthenticated, UnauthorizedClient, UnsupportedResponseType,
    };

    match error {
        ServerError(e) => Failure {
            error: FailureVariant::ServerError(e),
            redirect_uri: None,
            state: None,
        },

        TemporarilyUnavailable => Failure {
            error: FailureVariant::TemporarilyUnavailable,
            redirect_uri: None,
            state: None,
        },

        InvalidScope {
            redirect_uri,
            state,
        } => Failure {
            error: FailureVariant::InvalidScope,
            redirect_uri: Some(redirect_uri),
            state,
        },

        UnsupportedResponseType {
            redirect_uri,
            state,
        } => Failure {
            error: FailureVariant::UnsupportedResponseType,
            redirect_uri: Some(redirect_uri),
            state,
        },

        UnauthorizedClient => Failure {
            error: FailureVariant::UnauthorizedClient,
            redirect_uri: None,
            state: None,
        },

        AccessDenied {
            redirect_uri,
            state,
        } => Failure {
            error: FailureVariant::AccessDenied,
            redirect_uri: Some(redirect_uri),
            state,
        },

        InvalidRequest(e) => Failure {
            error: FailureVariant::InvalidRequest(e),
            redirect_uri: None,
            state: None,
        },

        Unauthenticated => Failure {
            error: FailureVariant::UnauthenticatedUser,
            redirect_uri: None,
            state: None,
        },
    }
    .into()
}
