//! rust-tdlib is a wrapper for [TDlib (Telegram Database library)](https://core.telegram.org/tdlib).
//! It allows you to make all the things that you can do with real telegram. So, yes, you can build your own telegram client using Rust language.
//!
//! First of all you have to initialize client. Your steps:
//! 1. use [WorkerBuilder](crate::client::worker::WorkerBuilder) to build a [Worker](crate::client::worker::Worker).
//! 2. start worker.
//! 3. initialize [TdlibParameters](crate::types::TdlibParameters) with two required parameters: api_id and api_hash.
//! 4. create new [Client](crate::client::client::Client) with [ClientBuilder](crate::client::client::ClientBuilder).
//! 5. [Authorize](crate::client::worker::Worker::auth_client) it with worker.
//! 6. write your own code to interact with Telegram.
//! ```
//! use rust_tdlib::{client::{Client, Worker}, tdjson, types::*};
//! #[tokio::main]
//! async fn main() {
//!     let mut worker = Worker::builder().build().unwrap();
//!     let waiter = worker.start();
//!     let tdlib_params = TdlibParameters::builder().api_id(env!("API_ID").parse::<i32>().unwrap()).api_hash(env!("API_HASH")).build();
//!     let client = rust_tdlib::client::Client::builder().with_tdlib_parameters(tdlib_params).build();
//!     let (client_state, client) = worker.bind_client(client1).await.unwrap();
//!     let me = client.get_me(GetMe::builder().build()).await.unwrap();
//!     println!("{:?}", me);
//! }
//! ```
//!
//! You can read all updates, received from Telegram server, such as: new messages, chats updates, new chats, user updates and so on. All updates varians declared within [Update](crate::types::Update).
//! ```
//! use rust_tdlib::{client::{Client, Worker}, types::{Update, TdlibParameters}};
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
#![allow(
    clippy::borrowed_box,
    clippy::upper_case_acronyms,
    clippy::field_reassign_with_default,
    clippy::manual_non_exhaustive
)]

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "tdjson")]
pub mod tdjson;

pub mod errors;
pub mod types;
