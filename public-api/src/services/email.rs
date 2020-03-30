use authmenow_public_app::contracts::{EmailMessage, EmailNotification};

#[derive(Clone)]
pub struct Email {}

impl Email {
    pub fn new() -> Self {
        Self {}
    }
}

impl EmailNotification for Email {
    fn send(&mut self, email: String, message: EmailMessage) -> bool {
        println!("EMAIL: send {:?} to {}", message, email);
        true
    }
}
