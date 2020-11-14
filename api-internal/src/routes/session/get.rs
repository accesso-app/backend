use crate::generated::components::{responses, schemas};
use crate::generated::paths::session_get::{Answer, Response};
use crate::session::Session;

pub async fn route(session: Session) -> Answer {
    Response::Ok(responses::SessionGetSuccess {
        user: schemas::SessionUser {
            first_name: session.user.first_name,
            last_name: session.user.last_name,
        },
    })
    .into()
}
