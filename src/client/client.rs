use super::observer::OBSERVER;
use async_trait::async_trait;
use rtdlib_sys::Tdlib;
use std::sync::{Arc, Condvar, Mutex};

use super::api::{Api, TdLibClient};
use crate::errors::RTDResult;
use crate::types::{
    AuthorizationState, AuthorizationStateWaitPhoneNumber, AuthorizationStateWaitRegistration,
};
use crate::{
    errors::RTDError,
    types::from_json,
    types::TdType,
    types::{
        AuthorizationStateWaitCode, AuthorizationStateWaitEncryptionKey,
        AuthorizationStateWaitPassword, CheckAuthenticationCode, CheckAuthenticationPassword,
        CheckDatabaseEncryptionKey, SetAuthenticationPhoneNumber, SetTdlibParameters,
        TdlibParameters, UpdateAuthorizationState,
    },
};
use std::io;
use tokio::{sync::mpsc, task::JoinHandle};

const CLOSED_CHANNEL_ERROR: RTDError = RTDError::Internal("channel closed");

/// `AuthStateHandler` trait provides methods that returns data, required for authentication
#[async_trait]
pub trait AuthStateHandler {
    /// Returns wait code
    async fn handle_wait_code(&self, wait_code: &AuthorizationStateWaitCode) -> String;
    /// Returns database encryption key
    async fn handle_encryption_key(
        &self,
        wait_encryption_key: &AuthorizationStateWaitEncryptionKey,
    ) -> String;
    /// Returns password
    async fn handle_wait_password(&self, wait_password: &AuthorizationStateWaitPassword) -> String;
    /// Returns phone number
    async fn handle_wait_phone_number(
        &self,
        wait_phone_number: &AuthorizationStateWaitPhoneNumber,
    ) -> String;
    async fn handle_wait_registration(
        &self,
        wait_registration: &AuthorizationStateWaitRegistration,
    ) -> String;
}

#[derive(Debug)]
pub enum ClientState {
    Closed,
    Error(String),
}

/// Provides minimal implementation of `AuthStateHandler`.
/// All required methods wait for stdin input
pub struct TypeInAuthStateHandler {}

impl TypeInAuthStateHandler {
    fn type_in() -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(e) => panic!("Can not get input value: {:?}", e),
        }
    }
}

#[async_trait]
impl AuthStateHandler for TypeInAuthStateHandler {
    async fn handle_wait_code(&self, _wait_code: &AuthorizationStateWaitCode) -> String {
        eprintln!("wait for auth code");
        TypeInAuthStateHandler::type_in()
    }

    async fn handle_encryption_key(
        &self,
        _wait_encryption_key: &AuthorizationStateWaitEncryptionKey,
    ) -> String {
        eprintln!("wait for encryption key");
        TypeInAuthStateHandler::type_in()
    }

    async fn handle_wait_password(
        &self,
        _wait_password: &AuthorizationStateWaitPassword,
    ) -> String {
        eprintln!("wait for password");
        TypeInAuthStateHandler::type_in()
    }

    async fn handle_wait_phone_number(
        &self,
        _wait_phone_number: &AuthorizationStateWaitPhoneNumber,
    ) -> String {
        eprintln!("wait for phone number");
        TypeInAuthStateHandler::type_in()
    }

    async fn handle_wait_registration(
        &self,
        _wait_registration: &AuthorizationStateWaitRegistration,
    ) -> String {
        unimplemented!()
    }
}

/// `Client` is a high-level abstraction of TDLib.
/// Before start any API interactions you must call `start().await`.
#[derive(Clone)]
pub struct Client<A, S>
where
    A: AuthStateHandler + Send + Sync + 'static,
    S: TdLibClient + Send + Sync + Clone + 'static,
{
    stop_flag: Arc<Mutex<bool>>,
    api: Api<S>,
    updates_sender: Option<mpsc::Sender<TdType>>,
    auth_state_handler: Arc<A>,
    tdlib_parameters: Arc<TdlibParameters>,
    have_auth: Arc<(Mutex<bool>, Condvar)>,
}

impl<A, S> Client<A, S>
where
    A: AuthStateHandler + Send + Sync + 'static,
    S: TdLibClient + Send + Sync + Clone + 'static,
{
    pub fn api(&self) -> &Api<S> {
        &self.api
    }

    pub fn new(api: S, auth_state_handler: A, tdlib_parameters: TdlibParameters) -> Self {
        // TODO: configure verbosity level
        Tdlib::set_log_verbosity_level(0).unwrap();
        // TODO: do we need stop flag in case of a lot of joins?
        let stop_flag = Arc::new(Mutex::new(false));
        Self {
            stop_flag,
            tdlib_parameters: Arc::new(tdlib_parameters),
            api: Api::new(api),
            auth_state_handler: Arc::new(auth_state_handler),
            have_auth: Arc::new((Mutex::new(false), Condvar::new())),
            updates_sender: None,
        }
    }

    /// If you want to receive Telegram updates (messages, channels, etc; see `crate::types::TdType`),
    /// you must set mpsc::Sender here.
    pub fn set_updates_sender(&mut self, updates_sender: mpsc::Sender<TdType>) {
        self.updates_sender = Some(updates_sender)
    }

    /// Starts interaction with TDLib.
    /// Method blocks until authorization performed.
    pub async fn start(&mut self) -> Result<JoinHandle<ClientState>, RTDError> {
        let (sx, mut rx) = mpsc::channel::<()>(2);
        let (auth_sx, auth_rx) = mpsc::channel::<UpdateAuthorizationState>(10);

        let handle = self.init_updates_task(auth_sx);
        let auth_handle = self.init_auth_task(sx, auth_rx);

        rx.recv().await.ok_or(CLOSED_CHANNEL_ERROR)?;
        Ok(tokio::spawn(async move {
            // TODO handle returned result properly
            tokio::select! {
                a = auth_handle => ClientState::Closed,
                u = handle => ClientState::Closed,
                r = rx.recv() => ClientState::Closed,
            }
        }))
    }

    pub(crate) fn init_updates_task(
        &self,
        mut auth_sx: mpsc::Sender<UpdateAuthorizationState>,
    ) -> JoinHandle<RTDResult<()>> {
        let api = self.api.clone();
        let stop_flag = self.stop_flag.clone();
        let mut updates_sender = self.updates_sender.clone();

        tokio::spawn(async move {
            let current = tokio::runtime::Handle::try_current().unwrap();
            while !*stop_flag.lock().unwrap() {
                let rec_api = api.raw_api().clone();
                if let Some(json) = current
                    // TODO: configure timeout
                    .spawn_blocking(move || rec_api.receive(2.0))
                    .await
                    .unwrap()
                {
                    trace!("received json from tdlib: {}", json);
                    match from_json::<TdType>(&json) {
                        Ok(t) => match OBSERVER.notify(t) {
                            None => {}
                            Some(t) => match t {
                                TdType::UpdateAuthorizationState(auth_state) => {
                                    trace!("auth state send: {:?}", auth_state);
                                    auth_sx
                                        .send(auth_state)
                                        .await
                                        .map_err(|_| CLOSED_CHANNEL_ERROR)?;
                                    trace!("auth state sent");
                                }
                                _ => match &mut updates_sender {
                                    None => {}
                                    Some(sender) => {
                                        trace!("update send: {:?}", t);
                                        sender.send(t).await.map_err(|_| CLOSED_CHANNEL_ERROR)?;
                                        trace!("update sent");
                                    }
                                },
                            },
                        },
                        Err(e) => {
                            panic!("{}", e)
                        }
                    };
                }
            }
            Ok(())
        })
    }

    fn init_auth_task(
        &self,
        sx: mpsc::Sender<()>,
        mut auth_rx: mpsc::Receiver<UpdateAuthorizationState>,
    ) -> JoinHandle<RTDResult<()>> {
        let auth_api = self.api.clone();
        let auth_state_handler = self.auth_state_handler.clone();
        let tdlib_params = self.tdlib_parameters.clone();
        tokio::spawn(async move {
            while let Some(auth_state) = auth_rx.recv().await {
                trace!("received new auth state: {:?}", auth_state);
                handle_auth_state(
                    &auth_api,
                    auth_state_handler.clone(),
                    auth_state,
                    sx.clone(),
                    tdlib_params.clone(),
                )
                .await?;
            }
            Ok(())
        })
    }
}

async fn handle_auth_state<A: AuthStateHandler, S: TdLibClient + Clone>(
    api: &Api<S>,
    auth_state_handler: Arc<A>,
    state: UpdateAuthorizationState,
    mut sender: mpsc::Sender<()>,
    tdlib_parameters: Arc<TdlibParameters>,
) -> RTDResult<()> {
    match state.authorization_state() {
        AuthorizationState::_Default(_) => Ok(()),
        AuthorizationState::Closed(_) => {
            todo!()
        }
        AuthorizationState::Closing(_) => {
            todo!()
        }
        AuthorizationState::LoggingOut(_) => {
            todo!()
        }
        AuthorizationState::Ready(_) => {
            trace!("ready state received, send signal");
            sender.send(()).await.map_err(|_| CLOSED_CHANNEL_ERROR)?;
            Ok(())
        }
        AuthorizationState::WaitCode(wait_code) => {
            let code = auth_state_handler.handle_wait_code(wait_code).await;
            api.check_authentication_code(CheckAuthenticationCode::builder().code(code).build())
                .await?;
            Ok(())
        }
        AuthorizationState::WaitEncryptionKey(wait_encryption_key) => {
            let key = auth_state_handler
                .handle_encryption_key(wait_encryption_key)
                .await;
            trace!("checking encryption key");
            api.check_database_encryption_key(
                CheckDatabaseEncryptionKey::builder()
                    .encryption_key(key)
                    .build(),
            )
            .await?;
            trace!("encryption key check done");
            Ok(())
        }
        AuthorizationState::WaitOtherDeviceConfirmation(_) => {
            todo!()
        }
        AuthorizationState::WaitPassword(wait_password) => {
            let password = auth_state_handler.handle_wait_password(wait_password).await;
            trace!("checking password");
            api.check_authentication_password(
                CheckAuthenticationPassword::builder()
                    .password(password)
                    .build(),
            )
            .await?;
            trace!("password checked");
            Ok(())
        }
        AuthorizationState::WaitPhoneNumber(wait_phone_number) => {
            let phone_number = auth_state_handler
                .handle_wait_phone_number(wait_phone_number)
                .await;
            api.set_authentication_phone_number(
                SetAuthenticationPhoneNumber::builder()
                    .phone_number(phone_number)
                    .build(),
            )
            .await?;
            Ok(())
        }
        AuthorizationState::WaitRegistration(_) => {
            todo!()
        }
        AuthorizationState::WaitTdlibParameters(_) => {
            api.set_tdlib_parameters(
                SetTdlibParameters::builder()
                    .parameters(tdlib_parameters)
                    .build(),
            )
            .await?;
            Ok(())
        }
        AuthorizationState::GetAuthorizationState(_) => {
            todo!()
        }
    }
}

