use serde::Serialize;

pub trait EmailNotification {
    fn send(&self, email: String, content: EmailMessage) -> bool;
}

#[derive(Debug, Serialize)]
pub enum EmailMessage {
    RegisterConfirmation {
        code: String,
    },
    RegisterFinished {
        first_name: String,
        last_name: String,
    },
}
