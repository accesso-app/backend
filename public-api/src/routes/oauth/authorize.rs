use crate::generated::{
    components::{request_bodies, responses},
    paths::oauth_authorize_request::Response,
};
use actix_swagger::Answer;
use actix_web::{dev, web, FromRequest, HttpMessage};
use authmenow_public_logic::models;

use responses::{
    OAuthAuthorizeDone as Success, OAuthAuthorizeRequestFailure as Failure,
    OAuthAuthorizeRequestFailureError as FailureVariant,
};

#[derive(Debug)]
pub struct Auth {
    user: Option<models::User>,
}

impl FromRequest for Auth {
    type Config = ();
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &actix_web::HttpRequest, _: &mut dev::Payload) -> Self::Future {
        use authmenow_public_logic::app::session::Session;

        if let Some(cookie) = req.cookie("session-token") {
            if let Some(app) = req.app_data::<web::Data<crate::App>>() {
                let app = app.read().unwrap();

                return match app.session_resolve(cookie.value().to_owned()) {
                    Err(_) => futures::future::ok(Auth { user: None }),
                    Ok(user) => futures::future::ok(Auth { user }),
                };
            } else {
                eprintln!("[Auth FromRequest] cannot resolve app data");
            }
        }

        futures::future::ok(Auth { user: None })
    }
}

pub async fn route(
    auth: Auth,
    body: web::Json<request_bodies::OAuthAuthorize>,
    app: web::Data<crate::App>,
) -> Answer<'static, Response> {
    use authmenow_public_logic::app::oauth::authorize::{
        OAuthAuthorize, RequestAuthCode,
        RequestAuthCodeFailed::{
            AccessDenied, InvalidRequest, InvalidScope, ServerError, TemporarilyUnavailable,
            Unauthenticated, UnauthorizedClient, UnsupportedResponseType,
        },
    };

    let form = RequestAuthCode {
        response_type: match body.response_type {
            request_bodies::OAuthAuthorizeResponseType::Code => "code".to_owned(),
        },
        client_id: body.client_id.clone(),
        redirect_uri: body.redirect_uri.clone(),
        scopes: body.scope.clone().map_or(vec![], |scope| {
            scope.split(' ').map(ToOwned::to_owned).collect()
        }),
        state: body.state.clone(),
    };

    let mut app = app.write().unwrap();

    match app.oauth_request_authorize_code(auth.user, form) {
        Err(ServerError) => Response::BadRequest(Failure {
            error: FailureVariant::ServerError,
            redirect_uri: None,
            state: None,
        }),

        Err(TemporarilyUnavailable) => Response::BadRequest(Failure {
            error: FailureVariant::TemporarilyUnavailable,
            redirect_uri: None,
            state: None,
        }),

        Err(InvalidScope {
            redirect_uri,
            state,
        }) => Response::BadRequest(Failure {
            error: FailureVariant::InvalidScope,
            redirect_uri: Some(redirect_uri),
            state,
        }),

        Err(UnsupportedResponseType {
            redirect_uri,
            state,
        }) => Response::BadRequest(Failure {
            error: FailureVariant::UnsupportedResponseType,
            redirect_uri: Some(redirect_uri),
            state,
        }),

        Err(UnauthorizedClient) => Response::BadRequest(Failure {
            error: FailureVariant::UnauthorizedClient,
            redirect_uri: None,
            state: None,
        }),

        Err(AccessDenied {
            redirect_uri,
            state,
        }) => Response::BadRequest(Failure {
            error: FailureVariant::AccessDenied,
            redirect_uri: Some(redirect_uri),
            state,
        }),

        Err(InvalidRequest) => Response::BadRequest(Failure {
            error: FailureVariant::InvalidRequest,
            redirect_uri: None,
            state: None,
        }),

        Err(Unauthenticated) => Response::BadRequest(Failure {
            error: FailureVariant::UnauthenticatedUser,
            redirect_uri: None,
            state: None,
        }),

        Ok(created) => Response::Ok(Success {
            redirect_uri: created.redirect_uri,
            code: created.code,
            state: created.state,
        }),
    }
    .answer()
}
