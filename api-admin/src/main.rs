#![deny(warnings)]
#![forbid(unsafe_code)]

use accesso_settings::Settings;
use actix_swagger::{Answer, StatusCode};
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_http::Response;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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
    _pool: web::Data<DbPool>,
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
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let settings = Settings::new("admin").expect("failed to parse settings");

    let manager = ConnectionManager::<PgConnection>::new(settings.database.connection_url());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let bind = settings.server.bind_address();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                actix_web::error::InternalError::from_response(
                    err,
                    Response::from(HttpResponse::BadRequest().json(AnswerFailure {
                        code: FailureCode::InvalidPayload,
                        message: None,
                    })),
                )
                .into()
            }))
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
    .await
}
