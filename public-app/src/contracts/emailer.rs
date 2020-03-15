pub trait Emailer {
    fn send(&self, email: String, content: String) -> bool;
}

pub trait RegisterEmailer: Emailer {
    fn confirmation_code(&self, email: String, code: String) -> bool {
        let content = format!("Enter this code {}", code);
        self.send(email, content)
    }
}
