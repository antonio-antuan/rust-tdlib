//! Module contains structs and traits, required for proper interaction with Telegram server.
#[doc(hidden)]
mod observer;

/// TDlib API methods.
#[allow(clippy::module_inception)]
pub mod client;
/// Handlers for all incoming data
pub mod worker;

#[doc(hidden)]
mod tdlib_client;

pub use client::Client;
pub use worker::{AuthStateHandler, ConsoleAuthStateHandler, Worker, WorkerBuilder};
