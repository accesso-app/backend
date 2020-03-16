#[macro_use]
extern crate validator_derive;

pub mod contracts;
pub mod models;
pub mod registrator;

#[derive(Clone)]
pub struct App<DB = (), E = (), G = ()> {
    pub db: DB,
    pub emailer: E,
    pub generator: G,
}
