// temporary #![deny(warnings)]
#![forbid(unsafe_code)]

mod cookie;
mod routes;
mod services;

use accesso_settings::Settings;

use accesso_app::Service;
use accesso_core::contracts::{EmailNotification, Repository, SecureGenerator};
use actix_swagger::{Answer, StatusCode};
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
struct SigninPayload {
    login: String,
    password: String,
}

#[derive(Serialize)]
#[serde(untagged)]
enum SigninResponse {
    Ok,
}

impl SigninResponse {
    #[inline]
    pub fn answer<'a>(self) -> Answer<'a, Self> {
        let status = match self {
            Self::Ok => StatusCode::OK,
        };

        Answer::new(self).status(status).content_type(None)
    }
}

async fn admin_signin(
    _app: web::Data<accesso_app::App>,
    _payload: web::Json<SigninPayload>,
) -> Answer<'static, SigninResponse> {
    SigninResponse::Ok.answer()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum FailureCode {
    InvalidPayload,
    InvalidRoute,
}

#[derive(Debug, Serialize)]
struct AnswerFailure {
    code: FailureCode,
    message: Option<String>,
}

async fn not_found(_req: HttpRequest) -> impl Responder {
    web::Json(AnswerFailure {
        code: FailureCode::InvalidRoute,
        message: None,
    })
    .with_status(StatusCode::NOT_FOUND)
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt::init();

    let settings = Settings::new("admin").expect("failed to parse settings");

    if settings.debug {
        tracing::info!("==> api-admin running in DEVELOPMENT MODE");
    } else {
        tracing::info!("==> PRODUCTION MODE in api-admin");
    }

    let db: Arc<dyn Repository> = Arc::new(
        accesso_db::Database::new(
            settings.database.connection_url(),
            settings.database.pool_size,
        )
        .await
        .expect("Failed to create database"),
    );

    let generator: Arc<dyn SecureGenerator> =
        Arc::new(accesso_core::services::generator::Generator::new());

    let emailer: Arc<dyn EmailNotification> = Arc::new(accesso_core::services::email::Email {
        api_key: settings.sendgrid.api_key,
        sender_email: settings.sendgrid.sender_email,
        application_host: settings.sendgrid.application_host,
        email_confirm_url_prefix: settings.sendgrid.email_confirm_url_prefix,
        email_confirm_template: settings.sendgrid.email_confirm_template,
    });

    let session_cookie_config = cookie::SessionCookieConfig {
        http_only: settings.cookies.http_only,
        secure: settings.cookies.secure,
        path: settings.cookies.path.clone(),
        name: settings.cookies.name.clone(),
    };

    let app = Arc::new(
        accesso_app::App::builder()
            .with_service(Service::from(db))
            .with_service(Service::from(emailer))
            .with_service(Service::from(generator))
            .build(),
    );

    let bind = settings.server.bind_address();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(app.clone()))
            .app_data(Data::new(session_cookie_config.clone()))
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(AnswerFailure {
                        code: FailureCode::InvalidPayload,
                        message: None,
                    }),
                )
                .into()
            }))
            //.wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("X-Frame-Options", "deny")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("X-XSS-Protection", "1; mode=block"),
            )
            .default_service(web::route().to(not_found))
            .service(web::resource("/admin/signin").route(web::post().to(admin_signin)))
    })
    .bind(&bind)?
    .run()
    .await?;

    Ok(())
}
