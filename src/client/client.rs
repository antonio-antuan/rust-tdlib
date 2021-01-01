use super::observer::OBSERVER;
use async_trait::async_trait;
use rtdlib_sys::Tdlib;
use std::sync::{Arc, Condvar, Mutex};

use super::api::{Api, TdLibClient};
use crate::errors::RTDResult;
use crate::types::{AuthorizationState, AuthorizationStateWaitPhoneNumber, AuthorizationStateWaitRegistration, AuthorizationStateWaitOtherDeviceConfirmation, RegisterUser};
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
    /// Interacts with provided link
    async fn handle_other_device_confirmation(&self, wait_device_confirmation: &AuthorizationStateWaitOtherDeviceConfirmation);
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
    ) -> (String, String);
}

#[derive(Debug)]
pub enum ClientState {
    Opened,
    Closed,
    Error(String),
}

/// Provides minimal implementation of `AuthStateHandler`.
/// All required methods wait for stdin input
pub struct ConsoleAuthStateHandler {}

impl ConsoleAuthStateHandler {
    fn wait_input() -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(e) => panic!("Can not get input value: {:?}", e),
        }
    }
}

#[async_trait]
impl AuthStateHandler for ConsoleAuthStateHandler {
    async fn handle_other_device_confirmation(&self, wait_device_confirmation: &AuthorizationStateWaitOtherDeviceConfirmation) {
        eprintln!("other device confirmation link: {}", wait_device_confirmation.link());
    }

    async fn handle_wait_code(&self, _wait_code: &AuthorizationStateWaitCode) -> String {
        eprintln!("wait for auth code");
        ConsoleAuthStateHandler::wait_input()
    }

    async fn handle_encryption_key(
        &self,
        _wait_encryption_key: &AuthorizationStateWaitEncryptionKey,
    ) -> String {
        eprintln!("wait for encryption key");
        ConsoleAuthStateHandler::wait_input()
    }

    async fn handle_wait_password(
        &self,
        _wait_password: &AuthorizationStateWaitPassword,
    ) -> String {
        eprintln!("wait for password");
        ConsoleAuthStateHandler::wait_input()
    }

    async fn handle_wait_phone_number(
        &self,
        _wait_phone_number: &AuthorizationStateWaitPhoneNumber,
    ) -> String {
        eprintln!("wait for phone number");
        ConsoleAuthStateHandler::wait_input()
    }

    async fn handle_wait_registration(
        &self,
        _wait_registration: &AuthorizationStateWaitRegistration,
    ) -> (String, String) {
        loop {
            eprintln!("waits for first_name and second_name separated by comma");
            let input: String = ConsoleAuthStateHandler::wait_input();
            let found: Vec<&str> = input.splitn(2, |c| c == ',').collect();
            match found.len() {
                2 => {
                    let f = found.get(0).unwrap().trim();
                    let s = found.get(1).unwrap().trim();
                    if f.len() > 0 && s.len() > 0 {
                        return (f.to_string(), s.to_string())
                    }
                }
                _ => { }
            }

        }
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
    // TODO: move to builder
    pub fn set_updates_sender(&mut self, updates_sender: mpsc::Sender<TdType>) {
        self.updates_sender = Some(updates_sender)
    }

    /// Starts interaction with TDLib.
    /// Method blocks until authorization performed.
    pub async fn start(&mut self) -> Result<JoinHandle<ClientState>, RTDError> {
        let (client_state_sx, mut client_state_rx) = mpsc::channel::<ClientState>(2);
        let (auth_sx, auth_rx) = mpsc::channel::<UpdateAuthorizationState>(10);

        let updates_handle = self.init_updates_task(auth_sx);
        let auth_handle = self.init_auth_task(client_state_sx, auth_rx);

        // wait until ClientState::Opened received
        while let Some(msg) = client_state_rx.recv().await {
            match msg {
                ClientState::Opened => {break}
                ClientState::Closed => {return Ok(tokio::spawn(async{ClientState::Closed}))}
                ClientState::Error(e) => {return Err(RTDError::TdlibError(e))}
            }
        }

        Ok(tokio::spawn(async move {
            tokio::select! {
                a = auth_handle => match a {
                    Ok(_) => return ClientState::Closed,
                    Err(e) => return ClientState::Error(e.to_string()),
                },
                u = updates_handle => match u {
                    Ok(_) => return ClientState::Closed,
                    Err(e) => return ClientState::Error(e.to_string()),
                },
                r = client_state_rx.recv() => match r {
                    Some(ClientState::Opened) => return ClientState::Error("received Opened state again".to_string()),
                    Some(state) => return state,
                    None => return ClientState::Error("auth state channel closed".to_string())
                },
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
        client_state_sx: mpsc::Sender<ClientState>,
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
                    client_state_sx.clone(),
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
    mut client_state_sx: mpsc::Sender<ClientState>,
    tdlib_parameters: Arc<TdlibParameters>,
) -> RTDResult<()> {
    match state.authorization_state() {
        AuthorizationState::_Default(_) => Ok(()),
        AuthorizationState::Closed(_) => {
            client_state_sx.send(ClientState::Closed).await.map_err(|_|CLOSED_CHANNEL_ERROR)?;
            Ok(())
        }
        AuthorizationState::Closing(_) => {
            Ok(())
        }
        AuthorizationState::LoggingOut(_) => {
            Ok(())
        }
        AuthorizationState::Ready(_) => {
            trace!("ready state received, send signal");
            client_state_sx.send(ClientState::Opened).await.map_err(|_| CLOSED_CHANNEL_ERROR)?;
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
        AuthorizationState::WaitOtherDeviceConfirmation(wait_device_confirmation) => {
            trace!("handing other device confirmation");
            auth_state_handler
                .handle_other_device_confirmation(wait_device_confirmation)
                .await;
            trace!("handled other device confirmation");
            Ok(())
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
        AuthorizationState::WaitRegistration(wait_registration) => {
            trace!("handling wait registration");
            let (first_name, last_name) = auth_state_handler
                .handle_wait_registration(wait_registration)
                .await;
            let register = RegisterUser::builder().first_name(first_name).last_name(last_name).build();
            api.register_user(register).await?;
            trace!("handled register user");
            Ok(())
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
            Err(RTDError::Internal("retrieved GetAuthorizationState update but observer not found any subscriber"))
        }
    }
}

