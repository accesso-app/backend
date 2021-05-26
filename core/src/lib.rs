#![deny(warnings)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate validator_derive;

pub mod app;
pub mod contracts;
pub mod models;

#[derive(Clone)]
pub struct App<DB = (), E = (), G = ()> {
    pub db: DB,
    pub emailer: E,
    pub generator: G,
}
