use accesso_core::app::extractors::AdminSessionToken;
use crate::generated::{
    paths::session_get::{Error, Response}
};
use actix_web::web::{self, Data};
use accesso_core::app::admin_session::{AdminSession, AdminSessionResolveError};
use crate::generated::components::responses::SessionGetSuccess;
use crate::generated::components::schemas::UserInfo;

pub async fn route(
    app: Data<accesso_app::App>,
    session_token: AdminSessionToken,
) -> Result<Response, Error> {
    let token = session_token.into_inner();
    let admin_user = app.admin_session_token_get(token)
        .await
        .map_err(session_token_get_error)?;

    if let Some(admin_user) = admin_user {
        Ok(Response::Ok(SessionGetSuccess {
            user_info: UserInfo {
                first_name: admin_user.first_name.to_string(),
                last_name: admin_user.last_name.to_string(),
            }
        }))
    } else {
        Err(Error::Unexpected(eyre::eyre!(
            "No token found"
        )))
    }
}

fn session_token_get_error(error: AdminSessionResolveError) -> Error {
    use AdminSessionResolveError::*;

    match error {
        Unauthorized => Error::Unauthorized,
        Unexpected(e) => Error::Unexpected(e),
    }
}
