use crate::generated::components::{responses, schemas};
use crate::generated::paths::session_get::{Error, Response};
use crate::session::Session;

pub async fn route(session: Session) -> Result<Response, Error> {
    Ok(Response::Ok(responses::SessionGetSuccess {
        user: schemas::SessionUser {
            first_name: session.user.first_name,
            last_name: session.user.last_name,
            email: session.user.email,
        },
    }))
}
