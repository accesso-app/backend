// https://github.com/actix/actix-web/blob/3a5b62b5502d8c2ba5d824599171bb381f6b1b49/examples/basic.rs

use actix_web::{
    dev::{AppService, Factory, HttpServiceFactory},
    http::{
        header::{IntoHeaderName, IntoHeaderValue},
        Cookie, Error as HttpError, HeaderName, HeaderValue, Method, StatusCode,
    },
    middleware, web, App, FromRequest, HttpRequest, HttpServer, Responder, Scope,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::future::Future;

// ========== ACTIX_SWAGGER CODE ========== //

// move to actix_swagger
#[derive(Debug)]
pub enum ContentType {
    Json,
    FormData,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Json => "application/json".to_string(),
            ContentType::FormData => "multipart/form-data".to_string(),
        }
    }
}

// extract to actix_swagger
pub struct Answer<'a, T> {
    response: T,
    status_code: Option<StatusCode>,
    cookies: Vec<Cookie<'a>>,
    headers: HashMap<String, HeaderValue>,
    content_type: Option<ContentType>,
}

impl<'a, T> Answer<'a, T> {
    pub fn new(response: T) -> Answer<'a, T> {
        Answer {
            response,
            status_code: None,
            cookies: vec![],
            headers: HashMap::new(),
            content_type: None,
        }
    }

    pub fn header<V>(mut self, key: String, value: V) -> Self
    where
        V: IntoHeaderValue,
    {
        if let Ok(value) = value.try_into() {
            self.headers.insert(key, value);
        }

        self
    }

    pub fn cookie(mut self, cookie: Cookie<'a>) -> Self {
        self.cookies.push(cookie);

        self
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status_code = Some(status);

        self
    }

    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);

        self
    }
}

use actix_http::Response;
use actix_web::Error;
use futures::future::{err, ok, Ready};

impl<'a, T: Serialize> Responder for Answer<'a, T> {
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        // parse self.content_type and stringify with it
        let body = match serde_json::to_string(&self.response) {
            Ok(body) => body,
            Err(e) => return err(e.into()),
        };

        let mut response = &mut Response::build(self.status_code.unwrap_or(StatusCode::OK));

        for (name, value) in self.headers {
            if let Some(header_name) = name.parse::<HeaderName>().ok() {
                response = response.header(header_name, value)
            }
        }

        for cookie in self.cookies {
            response = response.cookie(cookie);
        }

        ok(response.body(body))
    }
}

// ========== GENERATED CODE ========== //

// maybe move to actix_web
// struct PublicApi { api: Api }
struct PublicApi {
    root: Scope,
    routes: HashMap<String, Scope>,
}

impl PublicApi {
    pub fn new() -> Self {
        PublicApi {
            root: Scope::new("/"),
            routes: HashMap::new(),
        }
    }
}

impl HttpServiceFactory for PublicApi {
    fn register(mut self, config: &mut AppService) {
        let keys: Vec<String> = self.routes.keys().cloned().collect();

        for key in keys.iter() {
            if let Some(resource) = self.routes.remove(key) {
                self.root = self.root.service(resource);
            }
        }

        self.root.register(config);
    }
}

pub mod components {
    pub mod responses {
        use serde::{Deserialize, Serialize};

        /// User authenticated in Authmenow
        #[derive(Serialize, Deserialize)]
        pub struct UserAuthenticated {
            pub username: Option<String>,
            #[serde(rename = "displayName")]
            pub display_name: Option<String>,
        }

        #[derive(Serialize, Deserialize)]
        pub struct UserAnonymous {}
    }
}

pub mod paths {
    use super::{components, Answer, ContentType};
    use actix_http::Response;
    use actix_web::{http::StatusCode, Error, HttpRequest, Responder};
    use futures::future::{err, ok, Ready};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionGetResponse {
        Ok(components::responses::UserAuthenticated),
        NotAuthorized(components::responses::UserAnonymous),
    }

    impl SessionGetResponse {
        #[inline]
        pub fn answer<'a>(self) -> Answer<'a, Self> {
            let status = match self {
                Self::Ok(_) => StatusCode::OK,
                Self::NotAuthorized(_) => StatusCode::UNAUTHORIZED,
            };

            Answer::new(self)
                .status(status)
                .content_type(ContentType::Json)
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionCreateResponse {
        /// Session successfully created
        Ok,
    }

    impl SessionCreateResponse {
        #[inline]
        pub fn answer<'a>(self) -> Answer<'a, Self> {
            let status = match self {
                Self::Ok => StatusCode::OK,
            };

            Answer::new(self)
                .status(status)
                .content_type(ContentType::Json)
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionDeleteResponse {
        /// Session successfully deleted
        Ok,
    }

    impl SessionDeleteResponse {
        #[inline]
        pub fn answer<'a>(self) -> Answer<'a, Self> {
            let status = match self {
                Self::Ok => StatusCode::OK,
            };

            Answer::new(self)
                .status(status)
                .content_type(ContentType::Json)
        }
    }
}

impl PublicApi {
    fn bind<F, T, R, U>(mut self, path: String, method: Method, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        let scope_path = path.clone();
        take_mut::take(
            self.routes
                .entry(path)
                .or_insert_with(move || web::scope(scope_path.as_ref())),
            |scope| {
                scope.route(
                    "",
                    match method {
                        Method::DELETE => web::delete(),
                        Method::GET => web::get(),
                        Method::HEAD => web::head(),
                        Method::PATCH => web::patch(),
                        Method::POST => web::post(),
                        Method::PUT => web::put(),
                        _ => unimplemented!(),
                    }
                    .to(handler),
                )
            },
        );

        self
    }

    pub fn bind_session_get<F, T, R>(self, handler: F) -> Self
    where
        F: Factory<T, R, Answer<'static, paths::SessionGetResponse>>,
        T: FromRequest + 'static,
        R: Future<Output = Answer<'static, paths::SessionGetResponse>> + 'static,
    {
        self.bind("/session".to_owned(), Method::GET, handler)
    }

    pub fn bind_session_create<F, T, R>(self, handler: F) -> Self
    where
        F: Factory<T, R, Answer<'static, paths::SessionCreateResponse>>,
        T: FromRequest + 'static,
        R: Future<Output = Answer<'static, paths::SessionCreateResponse>> + 'static,
    {
        self.bind("/session".to_owned(), Method::POST, handler)
    }

    pub fn bind_session_delete<F, T, R>(self, handler: F) -> Self
    where
        F: Factory<T, R, Answer<'static, paths::SessionDeleteResponse>>,
        T: FromRequest + 'static,
        R: Future<Output = Answer<'static, paths::SessionDeleteResponse>> + 'static,
    {
        self.bind("/session".to_owned(), Method::DELETE, handler)
    }
}

impl Default for PublicApi {
    fn default() -> Self {
        let api = PublicApi::new();
        // add default handlers to response 501, if handler not binded
        api
    }
}

// ========== USER SPECIFIC CODE ========== //

#[derive(Debug, Serialize)]
struct AnswerFailure {
    code: i32,
    message: String,
}

async fn not_found(_req: HttpRequest) -> impl Responder {
    web::Json(AnswerFailure {
        code: 404,
        message: "Route not found".to_string(),
    })
    .with_status(StatusCode::NOT_FOUND)
}

async fn session_get() -> Answer<'static, paths::SessionGetResponse> {
    use components::responses::UserAuthenticated;
    use paths::SessionGetResponse;

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

async fn session_create() -> Answer<'static, paths::SessionCreateResponse> {
    paths::SessionCreateResponse::Ok.answer()
}

async fn session_delete() -> Answer<'static, paths::SessionDeleteResponse> {
    paths::SessionDeleteResponse::Ok.answer()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(not_found))
            .service(
                PublicApi::new()
                    .bind_session_get(session_get)
                    .bind_session_create(session_create)
                    .bind_session_delete(session_delete),
            )
    })
    .bind("127.0.0.1:9005")?
    .workers(1)
    .run()
    .await
}
