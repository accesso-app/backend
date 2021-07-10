use crate::generated::components::{parameters, responses};
use crate::generated::paths::viewer_get::{Error, Response};
use actix_web::web;

use accesso_core::app::session::{Session, SessionResolveError};
use responses::{
    ViewerGetFailure as Failure, ViewerGetFailureError as FailureError, ViewerGetSuccess as Success,
};

pub async fn route(
    access_token: parameters::AccessToken,
    app: web::Data<accesso_app::App>,
) -> Result<Response, Error> {
    let user = app
        .session_resolve_by_access_token(access_token.0)
        .await
        .map_err(map_session_resolve_error)?;

    if let Some(user) = user {
        Ok(Response::Ok(Success {
            first_name: user.first_name,
            last_name: user.last_name,
            id: user.id,
        }))
    } else {
        Err(Error::BadRequest(Failure {
            error: FailureError::Unauthorized,
        }))
    }
}

fn map_session_resolve_error(error: SessionResolveError) -> Error {
    use SessionResolveError::Unexpected;

    match error {
        Unexpected(e) => Error::Unexpected(e),
    }
}
