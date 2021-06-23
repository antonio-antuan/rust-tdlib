//! Module contains structs and traits, required for proper interaction with Telegram server.
#[doc(hidden)]
mod observer;

/// TDlib API methods.
#[allow(clippy::module_inception)]
pub mod client;
/// Handlers for all incoming data
pub mod worker;

/// Authorization state handlers.
pub mod auth_handler;

#[doc(hidden)]
pub mod tdlib_client;

pub use auth_handler::{AuthStateHandler, ConsoleAuthStateHandler, SignalAuthStateHandler};
pub use client::{Client, ClientState};
pub use worker::{Worker, WorkerBuilder};
