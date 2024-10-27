#![deny(clippy::all)]

extern crate napi;

#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate serde_derive;

pub mod bit_set;
pub mod error;
pub mod internal_magic_string;
pub mod locator;
pub mod magic_string;
pub mod result;
pub mod source_map;
pub mod utils;
