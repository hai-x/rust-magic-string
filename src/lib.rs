#![deny(clippy::all)]

extern crate napi;

#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate lazy_static;

pub mod error;
pub mod internal_magic_string;
pub mod magic_string;
pub mod utils;
