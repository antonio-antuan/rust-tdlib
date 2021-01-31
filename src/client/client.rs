//! Handlers for all incoming data
use super::observer::OBSERVER;
use async_trait::async_trait;
use rtdlib_sys::Tdlib;
use std::sync::Arc;

use super::api::{Api, RawApi, TdLibClient};
use crate::errors::RTDResult;
use crate::types::{
    AuthorizationState, AuthorizationStateWaitOtherDeviceConfirmation,
    AuthorizationStateWaitPhoneNumber, AuthorizationStateWaitRegistration, RegisterUser,
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
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::{sync::mpsc, task::JoinHandle};

const CLOSED_CHANNEL_ERROR: RTDError = RTDError::Internal("channel closed");

/// `AuthStateHandler` trait provides methods that returns data, required for authentication
///It allows you to handle particular "auth states", such as [WaitPassword](crate::types::AuthorizationStateWaitPassword), [WaitPhoneNumber](crate::types::AuthorizationStateWaitPhoneNumber) and so on.
#[async_trait]
pub trait AuthStateHandler {
    /// Interacts with provided link
    async fn handle_other_device_confirmation(
        &self,
        wait_device_confirmation: &AuthorizationStateWaitOtherDeviceConfirmation,
    );
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
    /// Returns first_name and second_name
    async fn handle_wait_registration(
        &self,
        wait_registration: &AuthorizationStateWaitRegistration,
    ) -> (String, String);
}

#[derive(Debug, Clone)]
pub enum ClientState {
    /// Client opened. You can start interaction
    Opened,
    /// Client closed properly. You must reopen it if you want to interact with Telegram
    Closed,
    /// Client closed with error
    Error(String),
}

/// Provides minimal implementation of `AuthStateHandler`.
/// All required methods wait for stdin input
#[derive(Debug, Clone)]
pub struct ConsoleAuthStateHandler;

impl Default for ConsoleAuthStateHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleAuthStateHandler {
    pub fn new() -> Self {
        Self
    }

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
    async fn handle_other_device_confirmation(
        &self,
        wait_device_confirmation: &AuthorizationStateWaitOtherDeviceConfirmation,
    ) {
        eprintln!(
            "other device confirmation link: {}",
            wait_device_confirmation.link()
        );
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
            if let 2 = found.len() {
                let f = found.get(0).unwrap().trim();
                let s = found.get(1).unwrap().trim();
                if !f.is_empty() && !s.is_empty() {
                    return (f.to_string(), s.to_string());
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ClientBuilder<A>
where
    A: AuthStateHandler + Send + Sync + 'static,
{
    tdlib_verbosity_level: i32,
    read_updates_timeout: f64,
    updates_sender: Option<mpsc::Sender<TdType>>,
    tdlib_parameters: Option<TdlibParameters>,
    auth_state_handler: A,
    tdlib: Tdlib,
    tdlib_log_file_path: Option<String>,
    tdlib_log_max_file_size: Option<i64>,
}

impl Default for ClientBuilder<ConsoleAuthStateHandler> {
    /// Provides default implementation with [ConsoleAuthStateHandler](crate::client::client::ConsoleAuthStateHandler)
    fn default() -> Self {
        Self {
            tdlib_verbosity_level: 0,
            tdlib_log_file_path: None,
            read_updates_timeout: 2.0,
            updates_sender: None,
            tdlib_parameters: None,
            auth_state_handler: ConsoleAuthStateHandler::new(),
            tdlib: Tdlib::new(),
            tdlib_log_max_file_size: None,
        }
    }
}

impl<A> ClientBuilder<A>
where
    A: AuthStateHandler + Send + Sync + 'static,
{
    /// TDlib log verbosity level
    /// See [tdlib method documentation](https://core.telegram.org/tdlib/docs/classtd_1_1_log.html#a0f683bd572154f7b4c8b4f973ea3395f) for details
    pub fn with_tdlib_verbosity_level(mut self, tdlib_verbosity_level: i32) -> Self {
        self.tdlib_verbosity_level = tdlib_verbosity_level;
        self
    }

    /// TDlib log file path
    /// See [tdlib method documentation](https://core.telegram.org/tdlib/docs/classtd_1_1_log.html#a038b57d66436f9f367f5c77360e8254b) for details.
    pub fn with_tdlib_log_file_path(mut self, tdlib_log_file_path: &str) -> Self {
        self.tdlib_log_file_path = Some(tdlib_log_file_path.to_string());
        self
    }

    /// TDlib log max file size
    /// See [tdlib method documentation](https://core.telegram.org/tdlib/docs/classtd_1_1_log.html#a749ea8521373bbe9f5c30f58bc591016) for details.
    pub fn with_tdlib_log_max_file_size(mut self, tdlib_log_max_file_size: i64) -> Self {
        self.tdlib_log_max_file_size = Some(tdlib_log_max_file_size);
        self
    }

    pub fn with_read_updates_timeout(mut self, read_updates_timeout: f64) -> Self {
        self.read_updates_timeout = read_updates_timeout;
        self
    }

    /// If you want to receive real-time updates (new messages, calls, etc.) you have to receive them with tokio::mpsc::Receiver<TdType>
    pub fn with_updates_sender(mut self, updates_sender: mpsc::Sender<TdType>) -> Self {
        self.updates_sender = Some(updates_sender);
        self
    }

    /// Base parameters for your TDlib instance.
    pub fn with_tdlib_parameters(mut self, tdlib_parameters: TdlibParameters) -> Self {
        self.tdlib_parameters = Some(tdlib_parameters);
        self
    }

    /// [AuthStateHandler](crate::client::client::AuthStateHandler) allows you to handle particular "auth states", such as [WaitPassword](crate::types::AuthorizationStateWaitPassword), [WaitPhoneNumber](crate::types::AuthorizationStateWaitPhoneNumber) and so on.
    /// See [AuthorizationState](crate::types::AuthorizationState).
    pub fn with_auth_state_handler<N>(self, auth_state_handler: N) -> ClientBuilder<N>
    where
        N: AuthStateHandler + Send + Sync + 'static,
    {
        ClientBuilder {
            auth_state_handler,
            tdlib_verbosity_level: self.tdlib_verbosity_level,
            read_updates_timeout: self.read_updates_timeout,
            updates_sender: self.updates_sender,
            tdlib_parameters: self.tdlib_parameters,
            tdlib: self.tdlib,
            tdlib_log_file_path: self.tdlib_log_file_path,
            tdlib_log_max_file_size: self.tdlib_log_max_file_size,
        }
    }

    pub fn build(self) -> RTDResult<Client<A, RawApi>> {
        if self.tdlib_parameters.is_none() {
            return Err(RTDError::InvalidParameters("tdlib_parameters not set"));
        };

        if self.tdlib_log_max_file_size.is_some() {
            Tdlib::set_log_max_file_size(self.tdlib_log_max_file_size.unwrap());
        }

        if self.tdlib_log_file_path.is_some() {
            Tdlib::set_log_file_path(Some(self.tdlib_log_file_path.unwrap().as_str()));
        }

        Tdlib::set_log_verbosity_level(self.tdlib_verbosity_level)
            .map_err(|e| RTDError::TdlibError(e.to_string()))?;
        let client = Client::new(
            RawApi::new(self.tdlib),
            self.auth_state_handler,
            self.tdlib_parameters.unwrap(),
            self.updates_sender,
            self.read_updates_timeout,
        );
        Ok(client)
    }
}

/// A high-level abstraction of TDLib.
/// Before start any API interactions you must call `start().await`.
#[derive(Debug, Clone)]
pub struct Client<A, S>
where
    A: AuthStateHandler + Send + Sync + 'static,
    S: TdLibClient + Send + Sync + Clone + 'static,
{
    stop_flag: Arc<AtomicBool>,
    api: Api<S>,
    updates_sender: Option<mpsc::Sender<TdType>>,
    auth_state_handler: Arc<A>,
    tdlib_parameters: Arc<TdlibParameters>,
    read_updates_timeout: f64,
}

impl Client<ConsoleAuthStateHandler, RawApi> {
    pub fn builder() -> ClientBuilder<ConsoleAuthStateHandler> {
        ClientBuilder::default()
    }
}

impl<A, S> Client<A, S>
where
    A: AuthStateHandler + Send + Sync + 'static,
    S: TdLibClient + Send + Sync + Clone + 'static,
{
    /// Returns instance of Api.
    /// Api implements all TDlib API-methods, such as "search_public_chat", "send_message" and so on.
    pub fn api(&self) -> &Api<S> {
        &self.api
    }

    // Client must be created only with builder
    pub(crate) fn new(
        api: S,
        auth_state_handler: A,
        tdlib_parameters: TdlibParameters,
        updates_sender: Option<mpsc::Sender<TdType>>,
        read_updates_timeout: f64,
    ) -> Self {
        let stop_flag = Arc::new(AtomicBool::new(false));
        Self {
            stop_flag,
            read_updates_timeout,
            updates_sender,
            tdlib_parameters: Arc::new(tdlib_parameters),
            api: Api::new(api),
            auth_state_handler: Arc::new(auth_state_handler),
        }
    }

    /// Starts interaction with TDLib.
    /// Method blocks until authorization performed.
    /// It returns [JoinHandle](tokio::task::JoinHandle) which allows you to handle client state.
    pub async fn start(&mut self) -> Result<JoinHandle<ClientState>, RTDError> {
        let (client_state_sx, mut client_state_rx) = mpsc::channel::<ClientState>(2);
        let (auth_sx, auth_rx) = mpsc::channel::<UpdateAuthorizationState>(10);

        let updates_handle = self.init_updates_task(auth_sx);
        let auth_handle = self.init_auth_task(client_state_sx, auth_rx);

        // wait until ClientState::Opened received
        if let Some(msg) = client_state_rx.recv().await {
            match msg {
                ClientState::Closed => return Ok(tokio::spawn(async { ClientState::Closed })),
                ClientState::Error(e) => return Err(RTDError::TdlibError(e)),
                ClientState::Opened => {}
            }
        }

        let stop = self.stop_flag.clone();

        Ok(tokio::spawn(async move {
            let res_state = tokio::select! {
                a = auth_handle => match a {
                    Ok(_) => ClientState::Closed,
                    Err(e) => ClientState::Error(e.to_string()),
                },
                u = updates_handle => match u {
                    Ok(_) => ClientState::Closed,
                    Err(e) => ClientState::Error(e.to_string()),
                },
                r = client_state_rx.recv() => match r {
                    Some(ClientState::Opened) => ClientState::Error("received Opened state again".to_string()),
                    Some(state) => state,
                    None => ClientState::Error("auth state channel closed".to_string())
                },
            };
            stop.store(true, Ordering::Release);
            res_state
        }))
    }

    /// Stops the client.
    /// You may want to await JoinHandle retrieved with `client.start().await` after stopping the client.
    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::Release)
    }

    // pub(crate) is just for unit-tests
    // It's the base routine: sends received updates to particular handlers: observer and auth_state handler
    pub(crate) fn init_updates_task(
        &self,
        auth_sx: mpsc::Sender<UpdateAuthorizationState>,
    ) -> JoinHandle<RTDResult<()>> {
        let api = self.api.clone();
        let stop_flag = self.stop_flag.clone();
        let mut updates_sender = self.updates_sender.clone();
        let recv_timeout = self.read_updates_timeout;

        tokio::spawn(async move {
            trace!("updates handler started");
            let current = tokio::runtime::Handle::try_current().unwrap();
            while !stop_flag.load(Ordering::Acquire) {
                let rec_api = api.raw_api().clone();
                if let Some(json) = current
                    .spawn_blocking(move || {
                        trace!("waiting for new updates");
                        let u = rec_api.receive(recv_timeout);
                        trace!("updates received");
                        u
                    })
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

    // created task handles [UpdateAuthorizationState][crate::types::UpdateAuthorizationState] and sends it to particular methods of specified [AuthStateHandler](crate::client::client::AuthStateHandler)
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
    client_state_sx: mpsc::Sender<ClientState>,
    tdlib_parameters: Arc<TdlibParameters>,
) -> RTDResult<()> {
    match state.authorization_state() {
        AuthorizationState::_Default(_) => Ok(()),
        AuthorizationState::Closed(_) => {
            client_state_sx
                .send(ClientState::Closed)
                .await
                .map_err(|_| CLOSED_CHANNEL_ERROR)?;
            Ok(())
        }
        AuthorizationState::Closing(_) => Ok(()),
        AuthorizationState::LoggingOut(_) => Ok(()),
        AuthorizationState::Ready(_) => {
            trace!("ready state received, send signal");
            client_state_sx
                .send(ClientState::Opened)
                .await
                .map_err(|_| CLOSED_CHANNEL_ERROR)?;
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
            let register = RegisterUser::builder()
                .first_name(first_name)
                .last_name(last_name)
                .build();
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
        AuthorizationState::GetAuthorizationState(_) => Err(RTDError::Internal(
            "retrieved GetAuthorizationState update but observer not found any subscriber",
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{AuthStateHandler, Client, ClientBuilder};
    use crate::types::*;
    use async_trait::async_trait;

    struct DummyStateHandler;
    #[async_trait]
    impl AuthStateHandler for DummyStateHandler {
        async fn handle_other_device_confirmation(
            &self,
            _: &AuthorizationStateWaitOtherDeviceConfirmation,
        ) {
            unimplemented!()
        }
        async fn handle_wait_code(&self, _: &AuthorizationStateWaitCode) -> String {
            unimplemented!()
        }

        async fn handle_encryption_key(&self, _: &AuthorizationStateWaitEncryptionKey) -> String {
            unimplemented!()
        }
        async fn handle_wait_password(&self, _: &AuthorizationStateWaitPassword) -> String {
            unimplemented!()
        }
        async fn handle_wait_phone_number(&self, _: &AuthorizationStateWaitPhoneNumber) -> String {
            unimplemented!()
        }
        async fn handle_wait_registration(
            &self,
            _: &AuthorizationStateWaitRegistration,
        ) -> (String, String) {
            unimplemented!()
        }
    }

    #[test]
    fn test_builder_auth_state_handler() {
        Client::builder()
            .with_tdlib_parameters(TdlibParameters::builder().build())
            .with_auth_state_handler(DummyStateHandler {})
            .build()
            .unwrap();
        ClientBuilder::default()
            .with_tdlib_parameters(TdlibParameters::builder().build())
            .with_auth_state_handler(DummyStateHandler {})
            .build()
            .unwrap();
        ClientBuilder::default()
            .with_tdlib_parameters(TdlibParameters::builder().build())
            .build()
            .unwrap();
    }

    #[test]
    fn test_builder_no_params() {
        let result = Client::builder().build();

        if result.is_ok() {
            panic!("client wrongly build without tdlib params")
        }

        Client::builder()
            .with_tdlib_parameters(TdlibParameters::builder().build())
            .build()
            .unwrap();
    }
}
