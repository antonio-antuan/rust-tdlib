//! Module contains structs and traits, required for proper interaction with Telegram server.
#[doc(hidden)]
mod observer;

/// TDlib API methods.
pub mod api;
#[allow(clippy::module_inception)]
/// Handlers for all incoming data
pub mod client;
pub mod errors;

pub use client::{AuthStateHandler, Client, ClientBuilder, ConsoleAuthStateHandler};
pub use rtdlib_sys::Tdlib;
