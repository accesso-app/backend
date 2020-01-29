use actix_web::{
    dev::{AppService, Factory, HttpServiceFactory},
    http::{header::IntoHeaderValue, Cookie, HeaderName, HeaderValue, Method, StatusCode},
    web, FromRequest, HttpRequest, Responder, Scope,
};
use serde::Serialize;
use std::collections::HashMap;
use std::future::Future;

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

pub struct Api {
    root: Scope,
    routes: HashMap<String, Scope>,
}

impl Api {
    pub fn new() -> Self {
        Api {
            root: Scope::new("/"),
            routes: HashMap::new(),
        }
    }

    pub fn bind<F, T, R, U>(mut self, path: String, method: Method, handler: F) -> Self
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
}

impl HttpServiceFactory for Api {
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
