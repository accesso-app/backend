#![deny(warnings)]
#![forbid(unsafe_code)]
#![allow(clippy::from_over_into)]

#[macro_use]
pub extern crate diesel;

mod implementation;
pub use chrono;
pub mod schema;

pub use implementation::Database;
