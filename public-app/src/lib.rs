pub mod contracts;
pub mod models;
pub mod register;

#[derive(Clone)]
pub struct App<DB = (), E = (), G = ()> {
    pub db: DB,
    pub emailer: E,
    pub generator: G,
}
