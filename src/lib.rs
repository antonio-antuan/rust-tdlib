#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

pub(crate) use rtdlib_sys::Tdlib;

pub mod client;

pub mod errors;
pub mod types;
