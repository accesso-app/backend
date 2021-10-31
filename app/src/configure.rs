use accesso_settings::Settings;
use actix_web::web::ServiceConfig;
use std::sync::Arc;
use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::health_service;
use actix_web::{http::StatusCode, web, HttpRequest, Responder};
use opentelemetry::sdk::Resource;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureCode {
    InvalidPayload,
    InvalidRoute,
    InvalidQueryParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerFailure {
    pub error: FailureCode,
    pub message: Option<String>,
}

pub async fn not_found(_req: HttpRequest) -> impl Responder {
    web::Json(AnswerFailure {
        error: FailureCode::InvalidRoute,
        message: None,
    })
    .with_status(StatusCode::NOT_FOUND)
}

pub fn install_logger(app_name: String, settings: &Settings) -> Result<WorkerGuard, eyre::Report> {
    use opentelemetry::sdk::trace;
    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    let env_filter = EnvFilter::try_from_default_env()?;
    LogTracer::init()?;

    let (writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    let bunyan_layer = BunyanFormattingLayer::new(app_name.clone(), move || writer.clone());

    if settings.use_opentelemetry {
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(std::env::var("OPENTELEMETRY_ENDPOINT_URL")?),
            )
            .with_trace_config(
                trace::config()
                    .with_resource(Resource::new(vec![KeyValue::new("service.name", app_name)])),
            )
            .install_batch(opentelemetry::runtime::Tokio)?;

        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        set_global_default(
            Registry::default()
                .with(telemetry)
                .with(JsonStorageLayer)
                .with(bunyan_layer)
                .with(env_filter),
        )?;
    } else {
        set_global_default(
            Registry::default()
                .with(JsonStorageLayer)
                .with(bunyan_layer)
                .with(env_filter),
        )?;
    }

    Ok(guard)
}

pub fn configure(config: &mut ServiceConfig, settings: Arc<Settings>) {
    use crate::Service;
    use accesso_core::contracts::{EmailNotification, Repository, SecureGenerator};
    use accesso_core::services;
    use actix_web::web::Data;
    use actix_web::HttpResponse;

    let db: Arc<dyn Repository> = Arc::new(accesso_db::Database::new(
        settings.database.connection_url(),
        settings.database.pool_size,
    ));

    let emailer: Arc<dyn EmailNotification> =
        Arc::new(services::Email::from(settings.sendgrid.clone()));

    let generator: Arc<dyn SecureGenerator> = Arc::new(services::Generator::new());

    let app = crate::App::builder()
        .with_service(Service::from(db))
        .with_service(Service::from(emailer))
        .with_service(Service::from(generator))
        .build();

    let session_cookie_config = crate::SessionCookieConfig {
        http_only: settings.cookies.http_only,
        secure: settings.cookies.secure,
        path: settings.cookies.path.clone(),
        name: settings.cookies.name.clone(),
    };

    config
        .app_data(Data::new(app))
        .app_data(Data::new(session_cookie_config))
        .app_data(web::JsonConfig::default().error_handler(|err, _| {
            let error_message = format!("{}", err);
            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(AnswerFailure {
                    error: FailureCode::InvalidPayload,
                    message: Some(error_message),
                }),
            )
            .into()
        }))
        .app_data(web::QueryConfig::default().error_handler(|err, _| {
            let error_message = format!("{}", err);
            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(AnswerFailure {
                    error: FailureCode::InvalidQueryParams,
                    message: Some(error_message),
                }),
            )
            .into()
        }))
        .service(health_service);
}
