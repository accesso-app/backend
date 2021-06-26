use crate::generated::components::{parameters, responses};
use crate::generated::paths::viewer_get::{Answer, Response};
use crate::lib::{blocking, EachResult};
use actix_web::web;

use responses::{
    ViewerGetFailure as Failure, ViewerGetFailureError as FailureError, ViewerGetSuccess as Success,
};

pub async fn route(
    access_token: parameters::AccessToken,
    app: web::Data<accesso_app::App>,
) -> Answer {
    use accesso_core::app::session::{Session, SessionResolveError::Unexpected};

    blocking(Response::Unexpected.answer(), move || {
        match app.session_resolve_by_access_token(access_token.0) {
            Err(Unexpected) => Response::Unexpected,
            Ok(None) => Response::BadRequest(Failure {
                error: FailureError::Unauthorized,
            }),
            Ok(Some(user)) => Response::Ok(Success {
                first_name: user.first_name,
                last_name: user.last_name,
                id: user.id,
            }),
        }
        .answer()
    })
    .await
    .get()
}
