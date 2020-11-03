use super::super::models::*;
use super::*;

#[derive(Default)]
pub struct DbMock {
    pub users: Vec<User>,
    pub register_requests: Vec<RegisterRequest>,
    pub session_tokens: Vec<SessionToken>,
    pub access_tokens: Vec<AccessToken>,
}

impl UserRepo for DbMock {
    fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError> {
        Ok(self
            .users
            .iter()
            .find(|user| user.email == email)
            .map_or(false, |_| true))
    }

    fn user_register(&mut self, form: UserRegisterForm) -> Result<User, RegisterUserError> {
        let has_email = self
            .user_has_with_email(form.email.clone())
            .map_err(|_| RegisterUserError::Unexpected)?;

        if has_email {
            Err(RegisterUserError::EmailAlreadyExists)
        } else {
            let user = User {
                id: form.id.clone(),
                email: form.email.clone(),
                password_hash: form.password_hash.clone(),
                first_name: form.first_name.clone(),
                last_name: form.last_name.clone(),
            };
            self.users.push(user.clone());
            Ok(user)
        }
    }

    fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<User>, UnexpectedDatabaseError> {
        Ok(self
            .users
            .iter()
            .find(|u| u.email == creds.email && u.password_hash == creds.password_hash)
            .map(|u| u.clone()))
    }
}

impl RequestsRepo for DbMock {
    fn register_request_save(
        &mut self,
        request: RegisterRequest,
    ) -> Result<RegisterRequest, SaveRegisterRequestError> {
        let code_exists = self
            .register_request_get_by_code(request.code.clone())
            .map_err(|_| SaveRegisterRequestError::Unexpected)?
            .map_or(false, |_| true);

        if code_exists {
            Err(SaveRegisterRequestError::CodeAlreadyExists)
        } else {
            self.register_requests.push(request.clone());
            Ok(request)
        }
    }

    /// Find actual register request by its code
    fn register_request_get_by_code(
        &self,
        code: String,
    ) -> Result<Option<RegisterRequest>, UnexpectedDatabaseError> {
        Ok(self
            .register_requests
            .iter()
            .find(|r| r.code == code)
            .map(|r| r.clone()))
    }

    fn register_requests_delete_all_for_email(
        &mut self,
        email: String,
    ) -> Result<usize, UnexpectedDatabaseError> {
        self.register_requests = self
            .register_requests
            .iter()
            .filter(|r| r.email != email)
            .map(|r| r.clone())
            .collect();

        Ok(self.register_requests.len())
    }
}

impl SessionRepo for DbMock {
    fn get_user_by_session_token(&self, token: String) -> Result<User, GetUserBySessionError> {
        let token = self
            .session_tokens
            .iter()
            .find(|t| t.token == token)
            .map(|t| t.clone());

        if let Some(token) = token {
            let user = self.users.iter().find(|u| u.id == token.user_id);

            if let Some(user) = user {
                Ok(user.clone())
            } else {
                Err(GetUserBySessionError::NotFound)
            }
        } else {
            Err(GetUserBySessionError::NotFound)
        }
    }

    fn get_user_by_access_token(&self, token: String) -> Result<User, GetUserBySessionError> {
        let token = self
            .access_tokens
            .iter()
            .find(|t| t.token == token)
            .map(|t| t.clone());

        if let Some(token) = token {
            let user = self.users.iter().find(|u| u.id == token.user_id);

            if let Some(user) = user {
                Ok(user.clone())
            } else {
                Err(GetUserBySessionError::NotFound)
            }
        } else {
            Err(GetUserBySessionError::NotFound)
        }
    }

    fn session_create(
        &mut self,
        session: SessionToken,
    ) -> Result<SessionToken, SessionCreateError> {
        let token = self
            .session_tokens
            .iter()
            .find(|t| t.token == session.token);

        if token.is_some() {
            Err(SessionCreateError::TokenAlreadyExists)
        } else {
            let user = self.users.iter().find(|u| u.id == session.user_id);

            if user.is_some() {
                self.session_tokens.push(session.clone());
                Ok(session)
            } else {
                Err(SessionCreateError::UserNotFound)
            }
        }
    }
}
