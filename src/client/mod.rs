mod observer;

pub mod api;
pub mod client;
pub mod errors;

pub use client::{Client, AuthStateHandler, TypeInAuthStateHandler};
pub use api::{RawApi, TdLibClient};
pub use rtdlib_sys::Tdlib;

