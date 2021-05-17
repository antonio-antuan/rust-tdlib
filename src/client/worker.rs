//! Handlers for all incoming data
use super::{
    client::{Client, ClientState},
    observer::OBSERVER,
    tdlib_client::{TdJson, TdLibClient},
};
use crate::types::Update;
use crate::{
    errors::RTDError,
    errors::RTDResult,
    tdjson::ClientId,
    types::{
        from_json, AuthorizationState, AuthorizationStateWaitCode,
        AuthorizationStateWaitEncryptionKey, AuthorizationStateWaitOtherDeviceConfirmation,
        AuthorizationStateWaitPassword, AuthorizationStateWaitPhoneNumber,
        AuthorizationStateWaitRegistration, CheckAuthenticationCode, CheckAuthenticationPassword,
        CheckDatabaseEncryptionKey, GetApplicationConfig, RObject, RegisterUser,
        SetAuthenticationPhoneNumber, SetTdlibParameters, TdType, UpdateAuthorizationState,
    },
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::{
    sync::{mpsc, RwLock},
    task::JoinHandle,
    time,
};

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
        println!(
            "other device confirmation link: {}",
            wait_device_confirmation.link()
        );
    }

    async fn handle_wait_code(&self, _wait_code: &AuthorizationStateWaitCode) -> String {
        println!("wait for auth code");
        ConsoleAuthStateHandler::wait_input()
    }

    async fn handle_encryption_key(
        &self,
        _wait_encryption_key: &AuthorizationStateWaitEncryptionKey,
    ) -> String {
        println!("wait for encryption key");
        ConsoleAuthStateHandler::wait_input()
    }

    async fn handle_wait_password(
        &self,
        _wait_password: &AuthorizationStateWaitPassword,
    ) -> String {
        println!("wait for password");
        ConsoleAuthStateHandler::wait_input()
    }

    async fn handle_wait_phone_number(
        &self,
        _wait_phone_number: &AuthorizationStateWaitPhoneNumber,
    ) -> String {
        println!("wait for phone number");
        ConsoleAuthStateHandler::wait_input()
    }

    async fn handle_wait_registration(
        &self,
        _wait_registration: &AuthorizationStateWaitRegistration,
    ) -> (String, String) {
        loop {
            println!("wait for first_name and second_name separated by comma");
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
pub struct WorkerBuilder<A, T>
where
    A: AuthStateHandler + Send + Sync + 'static,
    T: TdLibClient + Send + Sync + Clone + 'static,
{
    read_updates_timeout: f64,
    channels_send_timeout: f64,
    auth_state_handler: A,
    tdlib_client: T,
}

impl Default for WorkerBuilder<ConsoleAuthStateHandler, TdJson> {
    /// Provides default implementation with [ConsoleAuthStateHandler](crate::client::client::ConsoleAuthStateHandler)
    fn default() -> Self {
        Self {
            read_updates_timeout: 2.0,
            channels_send_timeout: 5.0,
            auth_state_handler: ConsoleAuthStateHandler::new(),
            tdlib_client: TdJson::new(),
        }
    }
}

impl<A, T> WorkerBuilder<A, T>
where
    A: AuthStateHandler + Send + Sync + 'static,
    T: TdLibClient + Send + Sync + Clone + 'static,
{
    /// Specifies timeout which will be used during sending to [tokio::sync::mpsc](tokio::sync::mpsc).
    pub fn with_channels_send_timeout(mut self, timeout: f64) -> Self {
        self.channels_send_timeout = timeout;
        self
    }

    pub fn with_read_updates_timeout(mut self, read_updates_timeout: f64) -> Self {
        self.read_updates_timeout = read_updates_timeout;
        self
    }

    /// [AuthStateHandler](crate::client::client::AuthStateHandler) allows you to handle particular "auth states", such as [WaitPassword](crate::types::AuthorizationStateWaitPassword), [WaitPhoneNumber](crate::types::AuthorizationStateWaitPhoneNumber) and so on.
    /// See [AuthorizationState](crate::types::AuthorizationState).
    pub fn with_auth_state_handler<N>(self, auth_state_handler: N) -> WorkerBuilder<N, T>
    where
        N: AuthStateHandler + Send + Sync + 'static,
    {
        WorkerBuilder {
            auth_state_handler,
            read_updates_timeout: self.read_updates_timeout,
            channels_send_timeout: self.channels_send_timeout,
            tdlib_client: self.tdlib_client,
        }
    }

    #[doc(hidden)]
    pub fn with_tdlib_client<C>(self, tdlib_client: C) -> WorkerBuilder<A, C>
    where
        C: TdLibClient + Send + Sync + Clone + 'static,
    {
        WorkerBuilder {
            tdlib_client,
            auth_state_handler: self.auth_state_handler,
            read_updates_timeout: self.read_updates_timeout,
            channels_send_timeout: self.channels_send_timeout,
        }
    }

    pub fn build(self) -> RTDResult<Worker<A, T>> {
        let worker = Worker::new(
            self.auth_state_handler,
            self.read_updates_timeout,
            self.channels_send_timeout,
            self.tdlib_client,
        );
        Ok(worker)
    }
}

type ClientsMap<S> = HashMap<ClientId, (Client<S>, mpsc::Sender<ClientState>)>;

/// The main object in all interactions.
/// You have to [start](crate::client::worker::Worker::start] worker and bind each client with worker using [auth_client](crate::client::worker::Worker::auth_client).
#[derive(Debug, Clone)]
pub struct Worker<A, S>
where
    A: AuthStateHandler + Send + Sync + 'static,
    S: TdLibClient + Send + Sync + Clone + 'static,
{
    run_flag: Arc<AtomicBool>,
    auth_state_handler: Arc<A>,
    read_updates_timeout: f64,
    channels_send_timeout: f64,
    tdlib_client: S,
    clients: Arc<RwLock<ClientsMap<S>>>,
}

impl Worker<ConsoleAuthStateHandler, TdJson> {
    pub fn builder() -> WorkerBuilder<ConsoleAuthStateHandler, TdJson> {
        WorkerBuilder::default()
    }
}

impl<A, T> Worker<A, T>
where
    A: AuthStateHandler + Send + Sync + 'static,
    T: TdLibClient + Send + Sync + Clone + 'static,
{
    /// Binds client with worker and runs authorization routines. Method returns error if worker is not running.
    pub async fn auth_client(
        &mut self,
        mut client: Client<T>,
    ) -> RTDResult<(JoinHandle<ClientState>, Client<T>)> {
        if !self.is_running() {
            return Err(RTDError::BadRequest("worker not started yet"));
        };
        let client_id = self.tdlib_client.new_client();
        log::debug!("new client created: {}", client_id);
        client.set_client_id(client_id)?;
        let (sx, mut rx) = mpsc::channel::<ClientState>(10);
        self.clients
            .write()
            .await
            .insert(client_id, (client.clone(), sx));
        log::debug!("new client added");

        client
            .get_application_config(GetApplicationConfig::builder().build())
            .await?;

        log::trace!("received first internal response");

        if let Some(msg) = rx.recv().await {
            match msg {
                ClientState::Closed => {
                    log::debug!("client state on auth: closed");
                    return Ok((tokio::spawn(async { ClientState::Closed }), client));
                }
                ClientState::Error(e) => {
                    log::debug!("client state on auth: error");
                    return Err(RTDError::TdlibError(e));
                }
                ClientState::Opened => {
                    log::debug!("client state on auth: opened");
                }
            }
        }
        let h = tokio::spawn(async move {
            match rx.recv().await {
                Some(ClientState::Opened) => {
                    ClientState::Error("received Opened state again".to_string())
                }
                Some(state) => state,
                None => ClientState::Error("auth state channel closed".to_string()),
            }
        });
        Ok((h, client))
    }

    /// Determines that the worker is running.
    pub fn is_running(&self) -> bool {
        self.run_flag.load(Ordering::Acquire)
    }

    #[cfg(test)]
    // Method needs for tests because we can't handle get_application_config request properly.
    pub async fn set_client(&mut self, mut client: Client<T>) -> Client<T> {
        let (sx, _) = mpsc::channel::<ClientState>(10);
        let cl = self.tdlib_client.new_client();
        self.clients.write().await.insert(cl, (client.clone(), sx));
        client.set_client_id(cl).unwrap();
        client
    }

    // Client must be created only with builder
    pub(crate) fn new(
        auth_state_handler: A,
        read_updates_timeout: f64,
        channels_send_timeout: f64,
        tdlib_client: T,
    ) -> Self {
        let run_flag = Arc::new(AtomicBool::new(false));
        let clients: ClientsMap<T> = HashMap::new();

        Self {
            run_flag,
            read_updates_timeout,
            tdlib_client,
            channels_send_timeout,
            auth_state_handler: Arc::new(auth_state_handler),
            clients: Arc::new(RwLock::new(clients)),
        }
    }

    /// Starts interaction with TDLib.
    /// It returns [JoinHandle](tokio::task::JoinHandle) which allows you to handle worker state.
    pub fn start(&mut self) -> JoinHandle<ClientState> {
        let (auth_sx, auth_rx) = mpsc::channel::<UpdateAuthorizationState>(20);

        self.run_flag.store(true, Ordering::Release);
        let updates_handle = self.init_updates_task(auth_sx);
        let auth_handle = self.init_auth_task(auth_rx);

        let run_flag = self.run_flag.clone();

        tokio::spawn(async move {
            let res_state = tokio::select! {
                a = auth_handle => match a {
                    Ok(_) => {
                        ClientState::Closed
                    },
                    Err(e) => ClientState::Error(e.to_string()),
                },
                u = updates_handle => match u {
                    Ok(_) => {
                        ClientState::Closed
                    },
                    Err(e) => ClientState::Error(e.to_string()),
                },
            };
            run_flag.store(false, Ordering::Release);
            res_state
        })
    }

    /// Stops the client.
    /// You may want to await JoinHandle retrieved with `client.start().await` after stopping the client.
    pub fn stop(&self) {
        self.run_flag.store(false, Ordering::Release)
    }

    // It's the base routine: sends received updates to particular handlers: observer and auth_state handler
    fn init_updates_task(
        &self,
        auth_sx: mpsc::Sender<UpdateAuthorizationState>,
    ) -> JoinHandle<RTDResult<()>> {
        let run_flag = self.run_flag.clone();
        let clients = self.clients.clone();
        let recv_timeout = self.read_updates_timeout;
        let send_timeout = time::Duration::from_secs_f64(self.channels_send_timeout);
        let tdlib_client = Arc::new(self.tdlib_client.clone());

        tokio::spawn(async move {
            let current = tokio::runtime::Handle::try_current().unwrap();
            while run_flag.load(Ordering::Acquire) {
                let cl = tdlib_client.clone();
                if let Some(json) = current
                    .spawn_blocking(move || cl.receive(recv_timeout))
                    .await
                    .unwrap()
                {
                    log::trace!("received json from tdlib: {}", json);
                    match from_json::<TdType>(&json) {
                        Ok(t) => match OBSERVER.notify(t) {
                            None => {}
                            Some(t) => {
                                if let TdType::Update(update) = t {
                                    if let Update::AuthorizationState(auth_state) = update {
                                        log::trace!("auth state send: {:?}", auth_state);
                                        auth_sx.send_timeout(auth_state, send_timeout).await?;
                                        log::trace!("auth state sent");
                                    } else if let Some(client_id) = update.client_id() {
                                        match clients.read().await.get(&client_id) {
                                            None => {
                                                log::warn!(
                                                    "found updates for unavailable client ({})",
                                                    client_id
                                                )
                                            }
                                            Some((client, _)) => {
                                                if let Some(sender) = client.updates_sender() {
                                                    log::trace!("sending update to client");
                                                    sender
                                                        .send_timeout(
                                                            Box::new(update),
                                                            send_timeout,
                                                        )
                                                        .await?;
                                                    log::trace!("update sent");
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Err(e) => return Err(e),
                    };
                }
            }
            Ok(())
        })
    }

    // created task handles [UpdateAuthorizationState][crate::types::UpdateAuthorizationState] and sends it to particular methods of specified [AuthStateHandler](crate::client::client::AuthStateHandler)
    fn init_auth_task(
        &self,
        mut auth_rx: mpsc::Receiver<UpdateAuthorizationState>,
    ) -> JoinHandle<RTDResult<()>> {
        let auth_state_handler = self.auth_state_handler.clone();
        let clients = self.clients.clone();
        let send_timeout = time::Duration::from_secs_f64(self.channels_send_timeout);

        tokio::spawn(async move {
            while let Some(auth_state) = auth_rx.recv().await {
                log::debug!("received new auth state: {:?}", auth_state);
                if let Some(client_id) = auth_state.client_id() {
                    match clients.read().await.get(&client_id) {
                        None => {
                            log::warn!("found auth updates for unavailable client ({})", client_id)
                        }
                        Some((client, auth_sender)) => {
                            handle_auth_state(
                                client,
                                auth_sender,
                                auth_state_handler.as_ref(),
                                auth_state,
                                send_timeout,
                            )
                            .await?;
                            log::debug!("state handled properly")
                        }
                    }
                }
            }
            Ok(())
        })
    }
}

async fn handle_auth_state<A: AuthStateHandler, R: TdLibClient + Clone>(
    client: &Client<R>,
    auth_sender: &mpsc::Sender<ClientState>,
    auth_state_handler: &A,
    state: UpdateAuthorizationState,
    send_state_timeout: time::Duration,
) -> RTDResult<()> {
    log::debug!("handling new auth state: {:?}", state);
    match state.authorization_state() {
        AuthorizationState::_Default => Ok(()),
        AuthorizationState::Closed(_) => {
            auth_sender
                .send_timeout(ClientState::Closed, send_state_timeout)
                .await?;
            Ok(())
        }
        AuthorizationState::Closing(_) => Ok(()),
        AuthorizationState::LoggingOut(_) => Ok(()),
        AuthorizationState::Ready(_) => {
            log::debug!("ready state received, send signal");
            auth_sender
                .send_timeout(ClientState::Opened, send_state_timeout)
                .await?;
            Ok(())
        }
        AuthorizationState::WaitCode(wait_code) => {
            let code = auth_state_handler.handle_wait_code(wait_code).await;
            client
                .check_authentication_code(CheckAuthenticationCode::builder().code(code).build())
                .await?;
            Ok(())
        }
        AuthorizationState::WaitEncryptionKey(wait_encryption_key) => {
            let key = auth_state_handler
                .handle_encryption_key(wait_encryption_key)
                .await;
            log::debug!("checking encryption key");
            client
                .check_database_encryption_key(
                    CheckDatabaseEncryptionKey::builder()
                        .encryption_key(key)
                        .build(),
                )
                .await?;
            log::debug!("encryption key check done");
            Ok(())
        }
        AuthorizationState::WaitOtherDeviceConfirmation(wait_device_confirmation) => {
            log::debug!("handling other device confirmation");
            auth_state_handler
                .handle_other_device_confirmation(wait_device_confirmation)
                .await;
            log::debug!("handled other device confirmation");
            Ok(())
        }
        AuthorizationState::WaitPassword(wait_password) => {
            let password = auth_state_handler.handle_wait_password(wait_password).await;
            log::debug!("checking password");
            client
                .check_authentication_password(
                    CheckAuthenticationPassword::builder()
                        .password(password)
                        .build(),
                )
                .await?;
            log::debug!("password checked");
            Ok(())
        }
        AuthorizationState::WaitPhoneNumber(wait_phone_number) => {
            let phone_number = auth_state_handler
                .handle_wait_phone_number(wait_phone_number)
                .await;
            client
                .set_authentication_phone_number(
                    SetAuthenticationPhoneNumber::builder()
                        .phone_number(phone_number)
                        .build(),
                )
                .await?;
            Ok(())
        }
        AuthorizationState::WaitRegistration(wait_registration) => {
            log::debug!("handling wait registration");
            let (first_name, last_name) = auth_state_handler
                .handle_wait_registration(wait_registration)
                .await;
            let register = RegisterUser::builder()
                .first_name(first_name)
                .last_name(last_name)
                .build();
            client.register_user(register).await?;
            log::debug!("handled register user");
            Ok(())
        }
        AuthorizationState::WaitTdlibParameters(_) => {
            log::debug!("going to set tdlib parameters");
            client
                .set_tdlib_parameters(
                    SetTdlibParameters::builder()
                        .parameters(client.tdlib_parameters())
                        .build(),
                )
                .await?;
            log::debug!("tdlib parameters set");
            Ok(())
        }
        AuthorizationState::GetAuthorizationState(_) => Err(RTDError::Internal(
            "retrieved GetAuthorizationState update but observer not found any subscriber",
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::client::client::Client;
    use crate::client::tdlib_client::TdLibClient;
    use crate::client::worker::Worker;
    use crate::errors::RTDResult;
    use crate::tdjson;
    use crate::types::{Chats, RFunction, RObject, SearchPublicChats, TdlibParameters};
    use std::time::Duration;
    use tokio::time::timeout;

    #[derive(Clone)]
    struct MockedRawApi {
        to_receive: Option<String>,
    }

    impl MockedRawApi {
        pub fn set_to_receive(&mut self, value: String) {
            log::trace!("delayed to receive: {}", value);
            self.to_receive = Some(value);
        }

        pub fn new() -> Self {
            Self { to_receive: None }
        }
    }

    impl TdLibClient for MockedRawApi {
        fn send<Fnc: RFunction>(&self, _client_id: tdjson::ClientId, _fnc: Fnc) -> RTDResult<()> {
            Ok(())
        }

        fn receive(&self, _timeout: f64) -> Option<String> {
            self.to_receive.clone()
        }

        fn execute<Fnc: RFunction>(&self, _fnc: Fnc) -> RTDResult<Option<String>> {
            unimplemented!()
        }

        fn new_client(&self) -> tdjson::ClientId {
            1
        }
    }

    #[tokio::test]
    async fn test_start_and_auth() {
        let mocked_raw_api = MockedRawApi::new();
        let mut worker = Worker::builder()
            .with_tdlib_client(mocked_raw_api.clone())
            .build()
            .unwrap();
        let res = timeout(
            Duration::from_millis(50),
            worker.auth_client(
                Client::builder()
                    .with_tdlib_client(mocked_raw_api.clone())
                    .with_tdlib_parameters(TdlibParameters::builder().build())
                    .build()
                    .unwrap(),
            ),
        )
        .await;
        match res {
            Err(e) => panic!("{:?}", e),
            Ok(v) => match v {
                Err(e) => assert_eq!(e.to_string(), "worker not started yet".to_string()),
                Ok(_) => panic!("error not raised"),
            },
        };

        worker.start();
        // we can't handle first request because we do not know @extra. so just wait a while.
        let res = timeout(
            Duration::from_millis(50),
            worker.auth_client(
                Client::builder()
                    .with_tdlib_client(mocked_raw_api.clone())
                    .with_tdlib_parameters(TdlibParameters::builder().build())
                    .build()
                    .unwrap(),
            ),
        )
        .await;
        match res {
            Err(_) => {}
            _ => panic!("error not raised"),
        };
    }

    #[tokio::test]
    async fn test_request_flow() {
        let mut mocked_raw_api = MockedRawApi::new();

        let search_req = SearchPublicChats::builder().build();
        let chats = Chats::builder().chat_ids(vec![1, 2, 3]).build();
        let chats: serde_json::Value = serde_json::to_value(chats).unwrap();
        let mut chats_object = chats.as_object().unwrap().clone();
        chats_object.insert(
            "@client_id".to_string(),
            serde_json::Value::Number(1.into()),
        );
        chats_object.insert(
            "@extra".to_string(),
            serde_json::Value::String(search_req.extra().unwrap().to_string()),
        );
        chats_object.insert(
            "@type".to_string(),
            serde_json::Value::String("chats".to_string()),
        );
        let to_receive = serde_json::to_string(&chats_object).unwrap();
        mocked_raw_api.set_to_receive(to_receive);
        log::trace!("chats objects: {:?}", chats_object);

        let mut worker = Worker::builder()
            .with_tdlib_client(mocked_raw_api.clone())
            .build()
            .unwrap();
        worker.start();

        let client = worker
            .set_client(
                Client::builder()
                    .with_tdlib_client(mocked_raw_api.clone())
                    .with_tdlib_parameters(TdlibParameters::builder().build())
                    .build()
                    .unwrap(),
            )
            .await;

        match timeout(
            Duration::from_secs(10),
            client.search_public_chats(search_req),
        )
        .await
        {
            Err(_) => panic!("did not receive response within 1 s"),
            Ok(Err(e)) => panic!("{}", e),
            Ok(Ok(result)) => assert_eq!(result.chat_ids(), &vec![1, 2, 3]),
        }
    }
}
