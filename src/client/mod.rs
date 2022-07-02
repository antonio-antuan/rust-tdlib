//! Module contains structs and traits, required for proper interaction with Telegram server.
#[doc(hidden)]
mod observer;

/// Handlers for all incoming data
pub mod worker;

#[doc(hidden)]
mod api;
/// Authorization state handlers.
pub mod auth_handler;

#[doc(hidden)]
pub mod tdlib_client;

pub use auth_handler::{AuthStateHandler, ConsoleAuthStateHandler, SignalAuthStateHandler};
use observer::OBSERVER;
use serde::de::DeserializeOwned;
pub use worker::{Worker, WorkerBuilder};

use crate::types::{Close, Ok, RFunction, TdlibParameters, Update};
use crate::{
    errors::{Error, Result},
    types::Error as TDLibError,
};
use tdlib_client::{TdJson, TdLibClient};
use tokio::sync::mpsc;

const CLIENT_NOT_AUTHORIZED: Error = Error::Internal("client not authorized yet");
const CLOSED_RECEIVER_ERROR: Error = Error::Internal("receiver already closed");
const INVALID_RESPONSE_ERROR: Error = Error::Internal("receive invalid response");
const NO_EXTRA: Error = Error::Internal("invalid tdlib response type, not have `extra` field");

/// Represents state of particular client instance.
#[derive(Debug, Clone, PartialEq)]
pub enum ClientState {
    /// Client opened. You can start interaction
    Opened,
    /// Client closed. You must reopen it if you want to interact with TDLib
    Closed,
    /// Client not authorizde yet
    Authorizing,
}

/// Struct stores all methods which you can call to interact with Telegram, such as:
/// [send_message](Api::send_message), [download_file](Api::download_file), [search_chats](Api::search_chats) and so on.
#[derive(Clone, Debug)]
pub struct Client<S>
where
    S: TdLibClient + Clone,
{
    tdlib_client: S,
    client_id: Option<i32>,
    is_started: bool,
    updates_sender: Option<mpsc::Sender<Box<Update>>>,
    tdlib_parameters: TdlibParameters,
    auth_state_channel_size: Option<usize>,
}

impl<S> Client<S>
where
    S: TdLibClient + Clone,
{
    pub(crate) fn get_auth_state_channel_size(&self) -> Option<usize> {
        self.auth_state_channel_size
    }

    pub(crate) fn tdlib_parameters(&self) -> &TdlibParameters {
        &self.tdlib_parameters
    }

    pub fn get_tdlib_client(&self) -> S {
        self.tdlib_client.clone()
    }

    pub(crate) fn get_client_id(&self) -> Result<i32> {
        match self.client_id {
            Some(client_id) => Ok(client_id),
            None => Err(CLIENT_NOT_AUTHORIZED),
        }
    }

    pub(crate) fn take_client_id(&mut self) -> Result<i32> {
        match self.client_id.take() {
            Some(client_id) => Ok(client_id),
            None => Err(CLIENT_NOT_AUTHORIZED),
        }
    }

    pub(crate) fn set_client_id(&mut self, client_id: i32) -> Result<()> {
        match self.client_id {
            Some(_) => Err(Error::BadRequest("client already authorized")),
            None => {
                self.client_id = Some(client_id);
                self.is_started = true;
                Ok(())
            }
        }
    }

    pub(crate) fn updates_sender(&self) -> &Option<mpsc::Sender<Box<Update>>> {
        &self.updates_sender
    }
}

#[derive(Debug)]
pub struct ClientBuilder<R>
where
    R: TdLibClient + Clone,
{
    updates_sender: Option<mpsc::Sender<Box<Update>>>,
    tdlib_parameters: Option<TdlibParameters>,
    tdlib_client: R,
    auth_state_channel_size: Option<usize>,
}

impl Default for ClientBuilder<TdJson> {
    fn default() -> Self {
        Self {
            updates_sender: None,
            tdlib_parameters: None,
            auth_state_channel_size: None,
            tdlib_client: TdJson::new(),
        }
    }
}

impl<R> ClientBuilder<R>
where
    R: TdLibClient + Clone,
{
    /// If you want to receive Telegram updates (messages, channels, etc; see `crate::types::Update`),
    /// you must set mpsc::Sender here.
    pub fn with_updates_sender(mut self, updates_sender: mpsc::Sender<Box<Update>>) -> Self {
        self.updates_sender = Some(updates_sender);
        self
    }

    /// If you want to receive all (AuthorizationState)[crate::types::authorization_state::AuthorizationState] changes
    /// you have to specify positive number of (channel)[tokio::sync::mpsc::channel] size.
    /// Channel will be used to send state changes.
    pub fn with_auth_state_channel(mut self, channel_size: usize) -> Self {
        self.auth_state_channel_size = Some(channel_size);
        self
    }

    /// Base parameters for your TDlib instance.
    pub fn with_tdlib_parameters(mut self, tdlib_parameters: TdlibParameters) -> Self {
        self.tdlib_parameters = Some(tdlib_parameters);
        self
    }

    #[doc(hidden)]
    pub fn with_tdlib_client<T: TdLibClient + Clone>(self, tdlib_client: T) -> ClientBuilder<T> {
        ClientBuilder {
            tdlib_client,
            updates_sender: self.updates_sender,
            tdlib_parameters: self.tdlib_parameters,
            auth_state_channel_size: self.auth_state_channel_size,
        }
    }

    pub fn build(self) -> Result<Client<R>> {
        if self.tdlib_parameters.is_none() {
            return Err(Error::BadRequest("tdlib_parameters not set"));
        };

        let client = Client::new(
            self.tdlib_client,
            self.updates_sender,
            self.tdlib_parameters.unwrap(),
            self.auth_state_channel_size,
        );
        Ok(client)
    }
}

impl Client<TdJson> {
    pub fn builder() -> ClientBuilder<TdJson> {
        ClientBuilder::default()
    }
}
/// TDLib high-level API methods.
/// Methods documentation can be found in https://core.telegram.org/tdlib/docs/td__api_8h.html
impl<R> Client<R>
where
    R: TdLibClient + Clone,
{
    #[doc(hidden)]
    pub fn new(
        tdlib_client: R,
        updates_sender: Option<mpsc::Sender<Box<Update>>>,
        tdlib_parameters: TdlibParameters,
        auth_state_channel_size: Option<usize>,
    ) -> Self {
        Self {
            tdlib_client,
            updates_sender,
            tdlib_parameters,
            auth_state_channel_size,
            is_started: false,
            client_id: None,
        }
    }

    pub fn set_updates_sender(&mut self, updates_sender: mpsc::Sender<Box<Update>>) -> Result<()> {
        match self.is_started {
            true => Err(Error::BadRequest(
                "can't set updates sender when client already started",
            )),
            false => {
                self.updates_sender = Some(updates_sender);
                Ok(())
            }
        }
    }

    /// Just a shortcut for `crate::client::client::Client::close`, allows you to stop the client.
    pub async fn stop(&self) -> Result<Ok> {
        self.close(Close::builder().build()).await
    }

    async fn make_request<T: RFunction, P: AsRef<T>, Q: DeserializeOwned>(
        &self,
        param: P,
    ) -> Result<Q> {
        let extra = param.as_ref().extra().ok_or(NO_EXTRA)?;
        let signal = OBSERVER.subscribe(extra);
        self.tdlib_client
            .send(self.get_client_id()?, param.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(extra);
        match received {
            Err(_) => Err(CLOSED_RECEIVER_ERROR),
            Ok(v) => {
                if error_received(&v) {
                    match serde_json::from_value::<TDLibError>(v) {
                        Ok(v) => Err(Error::TDLibError(v)),
                        Err(e) => {
                            log::error!("cannot deserialize error response: {:?}", e);
                            Err(INVALID_RESPONSE_ERROR)
                        }
                    }
                } else {
                    match serde_json::from_value::<Q>(v) {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            log::error!("response serialization error: {:?}", e);
                            Err(INVALID_RESPONSE_ERROR)
                        }
                    }
                }
            }
        }
    }
}

fn error_received(value: &serde_json::Value) -> bool {
    value.get("@type") == Some(&serde_json::Value::String("error".to_string()))
}
