use accesso_public_logic::contracts::{EmailMessage, EmailNotification};

#[derive(Clone)]
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
}

impl EmailNotification for Email {
    fn send(&mut self, email: String, message: EmailMessage) -> bool {
        println!("EMAIL: send {:?} to {}", message, email);

        match message {
            EmailMessage::RegisterConfirmation { code } => {
                let client = awc::Client::default();

                let req = client
                    .post("https://api.sendgrid.com/v3/mail/send")
                    .header("Authorization", format!("Bearer {}", self.api_key.clone()))
                    .send_json(&sg::MailSend {
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
                            to: vec![sg::Target {
                                email: email.clone(),
                            }],
                        }],
                    });

                actix_rt::spawn(async {
                    let resp = req.await;
                    println!("Email confirmation sent {:#?}", resp);
                });
            }
            _ => {}
        };

        true
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
