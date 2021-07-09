#![deny(warnings)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate validator_derive;

#[macro_use]
extern crate thiserror;

pub mod app;
pub mod contracts;
pub mod models;
pub mod services;
