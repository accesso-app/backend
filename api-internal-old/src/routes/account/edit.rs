use actix_web::web;

use accesso_core::app::account::AccountEditError;

use crate::generated::components::schemas::SessionUser;
use crate::generated::components::{request_bodies, responses};
use crate::generated::paths::account_edit as generated;
use crate::session::Session;

pub async fn route(
    body: web::Json<request_bodies::AccountEdit>,
    app: web::Data<accesso_app::App>,
    session: Session,
) -> Result<generated::Response, generated::Error> {
    use accesso_core::app::account::{Account, AccountEditForm};

    let form = AccountEditForm {
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
    };

    let user = app
        .account_edit(session.user.id, form)
        .await
        .map_err(map_error)?;

    Ok(generated::Response::Ok(responses::AccountEditSuccess {
        user: SessionUser {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        },
    }))
}

fn map_error(error: AccountEditError) -> generated::Error {
    use AccountEditError::{Unexpected, UserNotFound};

    match error {
        UserNotFound => generated::Error::BadRequest(responses::AccountEditFailure {
            error: responses::AccountEditFailureError::InvalidPayload,
        }),
        Unexpected(report) => generated::Error::Unexpected(report),
    }
}
