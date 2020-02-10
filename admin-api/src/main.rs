use actix_swagger::{Answer, ContentType, StatusCode};
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pool: web::Data<DbPool>,
    payload: web::Json<SigninPayload>,
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
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let listen_port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT");
    let connection_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connection_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let bind = format!("127.0.0.1:{}", listen_port);

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
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
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("X-Frame-Options", "deny")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("X-XSS-Protection", "1; mode=block"),
            )
            .default_service(web::route().to(not_found))
            .service(web::resource("/admin/signin").route(web::post().to(admin_signin)))
    })
    .bind(&bind)?;

    println!("Starting server on port {}", listen_port);

    server.run().await
}
