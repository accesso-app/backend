pub mod api {
    use crate::actix_swagger::{Answer, Api};
    use actix_web::{
        dev::{AppService, Factory, HttpServiceFactory},
        http::Method,
        FromRequest,
    };
    use std::future::Future;

    pub struct AuthmenowPublicApi {
        api: Api,
    }

    impl AuthmenowPublicApi {
        pub fn new() -> Self {
            AuthmenowPublicApi { api: Api::new() }
        }
    }

    impl HttpServiceFactory for AuthmenowPublicApi {
        fn register(self, config: &mut AppService) {
            self.api.register(config);
        }
    }

    impl AuthmenowPublicApi {
        pub fn bind_session_get<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::SessionGetResponse>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::SessionGetResponse>> + 'static,
        {
            self.api = self.api.bind("/session".to_owned(), Method::GET, handler);
            self
        }

        pub fn bind_session_create<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::SessionCreateResponse>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::SessionCreateResponse>> + 'static,
        {
            self.api = self.api.bind("/session".to_owned(), Method::POST, handler);
            self
        }

        pub fn bind_session_delete<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::SessionDeleteResponse>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::SessionDeleteResponse>> + 'static,
        {
            self.api = self
                .api
                .bind("/session".to_owned(), Method::DELETE, handler);
            self
        }
    }

    impl Default for AuthmenowPublicApi {
        fn default() -> Self {
            let api = AuthmenowPublicApi::new();
            // add default handlers to response 501, if handler not binded
            api
        }
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
    use super::components;
    use crate::actix_swagger::{Answer, ContentType};
    use actix_web::http::StatusCode;
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
