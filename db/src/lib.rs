#![deny(warnings)]
#![forbid(unsafe_code)]
#![allow(clippy::from_over_into)]

pub use chrono;

pub use implementation::Database;

mod entities;
mod implementation;
mod mappers;
mod repos;
mod sql_state;

#[macro_use]
extern crate async_trait;
