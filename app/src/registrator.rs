use crate::{App, Service};
use accesso_core::app::registrator::{
    CreateRegisterRequest, RegisterConfirmError, RegisterForm, RegisterRequestError, Registrator,
    RequestCreated,
};
use accesso_core::contracts::{
    EmailMessage, EmailNotification, Repository, SaveRegisterRequestError, SecureGenerator,
    UserRegisterForm,
};
use accesso_core::models::RegisterRequest;
use async_trait::async_trait;

use validator::Validate;

const MAX_CODE_INSERT_ATTEMPTS: u8 = 10;

#[async_trait]
impl Registrator for App {
    async fn registrator_create_request(
        &self,
        form: CreateRegisterRequest,
    ) -> Result<RequestCreated, RegisterRequestError> {
        let db = self.get::<Service<dyn Repository>>().unwrap();
        let generator = self.get::<Service<dyn SecureGenerator>>().unwrap();
        let emailer = self.get::<Service<dyn EmailNotification>>().unwrap();
        form.validate()?;

        let user_exists = db.user_has_with_email(form.email.clone()).await?;

        if user_exists {
            Err(RegisterRequestError::EmailAlreadyRegistered)
        } else {
            let mut generate_count = 0u8;

            let request: RegisterRequest = loop {
                generate_count += 1;

                let code = generator.confirmation_code();
                let request = RegisterRequest::new(form.email.clone(), code.clone());
                let result = db.register_request_save(request.clone()).await;

                if let Err(SaveRegisterRequestError::CodeAlreadyExists) = result {
                    if generate_count <= MAX_CODE_INSERT_ATTEMPTS {
                        continue;
                    }
                }

                break result;
            }?;

            emailer.send(
                form.email,
                EmailMessage::RegisterConfirmation {
                    code: request.code.clone(),
                },
            );

            Ok(RequestCreated {
                expires_at: request.expires_at,
            })
        }
    }

    async fn registrator_confirm(&self, form: RegisterForm) -> Result<(), RegisterConfirmError> {
        let db = self.get::<Service<dyn Repository>>().unwrap();
        let generator = self.get::<Service<dyn SecureGenerator>>().unwrap();
        let emailer = self.get::<Service<dyn EmailNotification>>().unwrap();

        form.validate()?;

        let code = form.confirmation_code.clone();

        match db.register_request_get_by_code(code).await? {
            Some(request) => {
                let password_hash = generator.password_hash(form.password).0;

                let created_user = db
                    .user_register(UserRegisterForm {
                        id: uuid::Uuid::new_v4(),
                        email: request.email,
                        password_hash,
                        first_name: form.first_name,
                        last_name: form.last_name,
                    })
                    .await?;

                emailer.send(
                    created_user.email.clone(),
                    EmailMessage::RegisterFinished {
                        first_name: created_user.first_name,
                        last_name: created_user.last_name,
                    },
                );

                db.register_requests_delete_all_for_email(created_user.email)
                    .await?;

                Ok(())
            }
            None => Err(RegisterConfirmError::CodeNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use accesso_core::contracts::*;
    use std::any::TypeId;
    use std::sync::Arc;

    fn mock_app<
        R: Repository + 'static,
        G: SecureGenerator + 'static,
        E: EmailNotification + 'static,
    >(
        db: R,
        generator: G,
        emailer: E,
    ) -> crate::App {
        let db: Arc<dyn Repository> = Arc::new(db);
        let db: Service<dyn Repository> = Service::from(db);

        let generator: Arc<dyn SecureGenerator> = Arc::new(generator);
        let generator: Service<dyn SecureGenerator> = Service::from(generator);

        let emailer: Arc<dyn EmailNotification> = Arc::new(emailer);
        let emailer: Service<dyn EmailNotification> = Service::from(emailer);

        println!(
            "typeid of db: {:?}",
            TypeId::of::<Service<dyn Repository>>()
        );
        crate::App::builder()
            .with_service(db)
            .with_service(emailer)
            .with_service(generator)
            .build()
    }

    #[actix_rt::test]
    async fn create_request_invalid_form() {
        let app = mock_app(
            MockDb::new(),
            MockSecureGenerator::new(),
            MockEmailNotification::new(),
        );
        let form = CreateRegisterRequest {
            email: "demo".to_owned(),
        };

        let result = app.registrator_create_request(form).await;

        assert_eq!(result, Err(RegisterRequestError::InvalidForm));
    }

    #[actix_rt::test]
    async fn create_request_user_exists() {
        let mut db = MockDb::new();
        db.users
            .expect_user_has_with_email()
            .returning(|_| Ok(true));

        let app = mock_app(db, MockSecureGenerator::new(), MockEmailNotification::new());

        println!("{:?}", &app);

        let form = CreateRegisterRequest {
            email: "demo@domain.com".to_owned(),
        };

        let result = app.registrator_create_request(form).await;

        assert_eq!(result, Err(RegisterRequestError::EmailAlreadyRegistered));
    }
}
