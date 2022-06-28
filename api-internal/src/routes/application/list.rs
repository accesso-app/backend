use actix_web::web::Data;

use accesso_core::app::application::{Application as _, ApplicationsListError};
use accesso_core::models;

use crate::generated::{
    components::{responses::ApplicationsListSuccess, schemas},
    paths::applications_list::{Error, Response},
};
use crate::session::Session;

pub async fn route(app: Data<accesso_app::App>, session: Session) -> Result<Response, Error> {
    let applications_list = app
        .applications_list(session.user.id)
        .await
        .map_err(map_applications_list_error)?;

    Ok(Response::Ok(ApplicationsListSuccess {
        available: applications_list
            .available
            .iter()
            .map(map_application)
            .collect(),
        installed: applications_list
            .installed
            .iter()
            .map(map_application)
            .collect(),
    }))
}

fn map_application(application: &models::Application) -> schemas::Application {
    schemas::Application {
        id: application.id,
        title: application.title.clone(),
        allowed_registrations: application.allowed_registrations,
        avatar: None,
    }
}

fn map_applications_list_error(error: ApplicationsListError) -> Error {
    match error {
        ApplicationsListError::Unexpected(report) => Error::InternalServerError(report.into()),
    }
}
