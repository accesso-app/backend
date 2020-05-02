#[macro_use]
extern crate validator_derive;

pub mod contracts;
pub mod models;
pub mod oauth_authorize;
pub mod registrator;
pub mod session;

#[derive(Clone)]
pub struct App<DB = (), E = (), G = ()> {
    pub db: DB,
    pub emailer: E,
    pub generator: G,
}
