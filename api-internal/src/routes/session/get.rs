use crate::generated::components::responses::SessionGetSuccess;
use crate::generated::components::schemas::SessionUser;
use crate::session::Session;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{post, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Response {
    Ok(SessionGetSuccess),
    Unauthorized,
    Unexpected,
}

impl Responder for Response {
    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        use Response::*;

        let mut response = HttpResponseBuilder::new(StatusCode::default());

        match self {
            Ok(json) => response.content_type(ContentType::json()).json(json),
            Unauthorized => response.status(StatusCode::UNAUTHORIZED).finish(),
            Unexpected => response.status(StatusCode::INTERNAL_SERVER_ERROR).finish(),
        }
    }
}

#[post("/session/get", name = "sessionGet")]
pub async fn route(session: Session) -> impl Responder {
    Response::Ok(SessionGetSuccess {
        user: SessionUser {
            first_name: session.user.first_name,
            last_name: session.user.last_name,
        },
    })
}
