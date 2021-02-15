//! rust-tdlib is a wrapper for [TDlib (Telegram Database library)](https://core.telegram.org/tdlib).
//! It allows you to make all the things that you can do with real telegram. So, yes, you can build your own telegram client using Rust language.
//!
//! First of all you have to initialize client. Your steps:
//! 1. initialize [TdlibParameters](crate::types::TdlibParameters) with two required parameters: api_id and api_hash.
//! 2. use [ConsoleAuthStateHandler](crate::client::ConsoleAuthStateHandler) with default builder or write you own [AuthStateHandler](crate::client::AuthStateHandler).
//! 3. use them as parameters of a [ClientBuilder](crate::client::ClientBuilder).
//! 4. initialize [Worker](crate::client::worker::Worker) and start it.
//! 5. [Authorize](crate::client::worker::Worker::auth_client) client with worker.
//! 6. write your own code to interact with Telegram.
//! ```
//! use rust_tdlib::{client::{Client, Worker}, tdjson, types::*};
//! #[tokio::main]
//! async fn main() {
//!     let tdlib_params = TdlibParameters::builder().api_id(env!("API_ID").parse::<i32>().unwrap()).api_hash(env!("API_HASH")).build();
//!     let client = rust_tdlib::client::Client::builder().with_tdlib_parameters(tdlib_params).build();
//!     let mut worker = Worker::builder().build().unwrap();
//!     let waiter = worker.start();
//!     let (client_state, client) = worker.auth_client(client1).await.unwrap();
//!     let me = client.get_me(GetMe::builder().build()).await.unwrap();
//!     println!("{:?}", me);
//! }
//! ```
//!
//! You can read all updates, received from Telegram server, such as: new messages, chats updates, new chats, user updates and so on. All updates varians declared within [Update](crate::types::Update).
//! ```
//! use rust_tdlib::{client::Client, types::{Update, TdlibParameters}};
//! #[tokio::main]
//! async fn main() {
//!     let (sender, mut receiver) = tokio::sync::mpsc::channel::<Update>(10);
//!     let tdlib_params = TdlibParameters::builder().api_id(env!("API_ID").parse::<i32>().unwrap()).api_hash(env!("API_HASH")).build();
//!     let client = rust_tdlib::client::Client::builder().with_tdlib_parameters(tdlib_params).with_updates_sender(sender).build();
//!     let mut worker = Worker::builder().build().unwrap();
//!     let waiter = worker.start();
//!     let (client_state, client) = worker.auth_client(client).await.unwrap();
//!     if let Some(message) = receiver.recv().await.unwrap() {
//!         eprintln!("updates handler received {:?}", message);
//!     }
//! }
//! ```
#![allow(clippy::large_enum_variant, clippy::borrowed_box)]

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "client")]
#[macro_use]
extern crate log;

#[cfg(feature = "client")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "tdjson")]
pub mod tdjson;

pub mod errors;
pub mod types;
