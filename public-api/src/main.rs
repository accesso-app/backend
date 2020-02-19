// https://github.com/actix/actix-web/blob/3a5b62b5502d8c2ba5d824599171bb381f6b1b49/examples/basic.rs

#[macro_use]
extern crate diesel;

use actix_swagger::Answer;
use actix_web::{
    http::{Cookie, StatusCode},
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod generated;

async fn session_get(b: HttpRequest) -> Answer<'static, generated::paths::SessionGetResponse> {
    use generated::components::responses::UserAuthenticated;
    use generated::paths::SessionGetResponse;

    println!("{:#?}", b.headers());

    SessionGetResponse::Ok(UserAuthenticated {
        username: Some(String::from("sergeysova")),
        display_name: Some(String::from("🦉")),
    })
    .answer()
    .header("x-csrf-token".to_string(), "DEEEEEEEEMO")
    .cookie(
        Cookie::build("CSRF-Token", "HopHey")
            .secure(true)
            .http_only(true)
            .finish(),
    )
    .cookie(Cookie::build("demo", "value").finish())
}

async fn session_create() -> Answer<'static, generated::paths::SessionCreateResponse> {
    generated::paths::SessionCreateResponse::Ok.answer()
}

async fn session_delete() -> Answer<'static, generated::paths::SessionDeleteResponse> {
    generated::paths::SessionDeleteResponse::Ok.answer()
}

mod models {
    use authmenow_db::schema::clients;

    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
    pub struct Client {
        pub id: uuid::Uuid,
        pub redirect_uri: Vec<String>,
        pub secret_key: String,
        pub scopes: Vec<String>,
        pub title: String,
    }
}

fn clients_find_by_id(
    uid: uuid::Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Client>, diesel::result::Error> {
    use authmenow_db::schema::clients::dsl::*;
    use diesel::prelude::*;

    let client = clients
        .filter(id.eq(uid))
        .first::<models::Client>(conn)
        .optional()?;

    Ok(client)
}

use generated::paths::oauth_authorize_request as authreq;
async fn oauth_authorize_request(
    query: authreq::Query,
    pool: web::Data<DbPool>,
) -> Answer<'static, authreq::Response> {
    match handle_authorize(query.client_id, pool) {
        Err(AuthorizeError::ClientNotFound) => authreq::Response::NotFound.answer(),
        Err(AuthorizeError::UnexpectedError) => authreq::Response::InternalServerError.answer(),
        Ok(_) => authreq::Response::SeeOther
            .answer()
            .header("Location".to_owned(), "https://google.com"),
    }
}

enum AuthorizeError {
    ClientNotFound,
    UnexpectedError,
}

impl From<diesel::result::Error> for AuthorizeError {
    fn from(error: diesel::result::Error) -> AuthorizeError {
        use diesel::result::Error;
        match error {
            Error::NotFound => AuthorizeError::ClientNotFound,
            _ => AuthorizeError::UnexpectedError,
        }
    }
}

fn handle_authorize(client_id: uuid::Uuid, pool: web::Data<DbPool>) -> Result<(), AuthorizeError> {
    clients_find_by_id(client_id, &pool.get().unwrap())?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum FailureCode {
    InvalidPayload,
    InvalidRoute,
    InvalidQueryParams,
}

#[derive(Debug, Serialize, Deserialize)]
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
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let listen_port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT");
    let connection_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connection_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let bind = format!("127.0.0.1:{}", listen_port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                let error_message = format!("{}", err);
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(AnswerFailure {
                        code: FailureCode::InvalidPayload,
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
                        code: FailureCode::InvalidQueryParams,
                        message: Some(error_message),
                    }),
                )
                .into()
            }))
            .wrap(
                middleware::DefaultHeaders::new()
                    // .header("X-Frame-Options", "deny")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("X-XSS-Protection", "1; mode=block"),
            )
            .default_service(web::route().to(not_found))
            .service(
                generated::api::AuthmenowPublicApi::new()
                    .bind_session_get(session_get)
                    .bind_session_create(session_create)
                    .bind_session_delete(session_delete)
                    .bind_oauth_authorize_request(oauth_authorize_request),
            )
    })
    .bind(bind)?
    .run()
    .await
}
