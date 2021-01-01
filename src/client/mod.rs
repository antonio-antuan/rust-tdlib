mod observer;

pub mod api;
#[allow(clippy::module_inception)]
pub mod client;
pub mod errors;

pub use client::{AuthStateHandler, Client, ClientBuilder, ConsoleAuthStateHandler};
pub use rtdlib_sys::Tdlib;
