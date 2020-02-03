use actix_swagger::{Answer, ContentType};
use actix_web::http::StatusCode;
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
    Created,
}

impl SigninResponse {
    #[inline]
    pub fn answer<'a>(self) -> Answer<'a, Self> {
        let status = match self {
            Self::Created => StatusCode::CREATED,
        };

        Answer::new(self)
            .status(status)
            .content_type(ContentType::Json)
    }
}

async fn user_signin(
    pool: web::Data<DbPool>,
    payload: web::Json<SigninPayload>,
) -> Answer<'static, SigninResponse> {
    SigninResponse::Created.answer()
}

#[derive(Debug, Serialize)]
struct AnswerFailure {
    code: i32,
    message: String,
}

async fn not_found(_req: HttpRequest) -> impl Responder {
    web::Json(AnswerFailure {
        code: 404,
        message: "route_not_found".to_string(),
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
            .data(web::JsonConfig::default().error_handler(|err, req| {
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(AnswerFailure {
                        code: 400,
                        message: "invalid_json".to_owned(),
                    }),
                )
                .into()
            }))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .default_service(web::route().to(not_found))
            .service(web::resource("/user/signin").route(web::post().to(user_signin)))
    })
    .bind(&bind)?;

    println!("Starting server on port {}", listen_port);

    server.run().await
}
