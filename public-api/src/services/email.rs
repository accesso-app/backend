use authmenow_public_app::contracts::{Emailer, RegisterEmailer};

#[derive(Clone)]
pub struct Email {}

impl Email {
    pub fn new() -> Self {
        Self {}
    }
}

impl Emailer for Email {
    fn send(&self, email: String, content: String) -> bool {
        println!("EMAIL: send {} to {}", content, email);
        true
    }
}

impl RegisterEmailer for Email {}
