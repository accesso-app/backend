use crate::contracts::{EmailMessage, EmailNotification, SendEmailError};
use accesso_settings::SendGrid;
use async_trait::async_trait;
use reqwest::{Client, StatusCode};

#[derive(Clone, Debug)]
pub struct Email {
    /// SendGrid api_key
    pub api_key: String,

    /// Who sends email: no-reply@domain.com
    pub sender_email: String,

    /// domain of the application, without https:// and any path
    pub application_host: String,

    /// Confirmation url prefix. Should be concatenated with https:// and application_host
    pub email_confirm_url_prefix: String,
    pub email_confirm_template: String,
    pub enabled: bool,
    client: Client,
}

impl From<SendGrid> for Email {
    fn from(s: SendGrid) -> Self {
        Self {
            api_key: s.api_key,
            sender_email: s.sender_email,
            application_host: s.application_host,
            email_confirm_template: s.email_confirm_template,
            email_confirm_url_prefix: s.email_confirm_url_prefix,
            enabled: s.enabled,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl EmailNotification for Email {
    #[tracing::instrument]
    async fn send(&self, email: String, message: EmailMessage) -> Result<(), SendEmailError> {
        if !self.enabled {
            tracing::warn!("Email service is disabled!");
            tracing::debug!(?message);
            return Ok(());
        }

        if let EmailMessage::RegisterConfirmation { code } = message {
            let request = self
                .client
                .post("https://api.sendgrid.com/v3/mail/send")
                .header("Authorization", format!("Bearer {}", self.api_key.clone()))
                .json(&sg::MailSend {
                    subject: "Confirm registration at Accesso".to_owned(),
                    template_id: self.email_confirm_template.clone(),
                    from: sg::Sender {
                        email: self.sender_email.clone(),
                        name: "Accesso".to_owned(),
                    },
                    personalizations: vec![sg::Personalization {
                        dynamic_template_data: sg::TemplateData {
                            application_host: self.application_host.clone(),
                            confirm_registration_url: format!(
                                "https://{host}{prefix}{code}",
                                host = self.application_host,
                                prefix = self.email_confirm_url_prefix,
                                code = code
                            ),
                        },
                        to: vec![sg::Target { email }],
                    }],
                });

            let resp = request.send().await?;

            tracing::info!("resp: {:?}", resp);

            if resp.status() != StatusCode::ACCEPTED {
                return Err(SendEmailError::Unexpected(eyre::eyre!(
                    "Could not send email!, status: {}",
                    resp.status()
                )));
            }
        }

        Ok(())
    }
}

mod sg {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct MailSend {
        pub personalizations: Vec<Personalization>,
        pub from: Sender,
        pub subject: String,
        pub template_id: String,
    }

    #[derive(Debug, Serialize)]
    pub struct Personalization {
        pub to: Vec<Target>,
        pub dynamic_template_data: TemplateData,
    }

    #[derive(Debug, Serialize)]
    pub struct Target {
        pub email: String,
    }

    #[derive(Debug, Serialize)]
    pub struct TemplateData {
        #[serde(rename = "applicationHost")]
        pub application_host: String,

        #[serde(rename = "confirmRegistrationUrl")]
        pub confirm_registration_url: String,
    }

    #[derive(Debug, Serialize)]
    pub struct Sender {
        pub email: String,
        pub name: String,
    }
}
