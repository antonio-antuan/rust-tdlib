//! Handlers for all incoming data
use super::{
    auth_handler::{AuthStateHandler, ConsoleAuthStateHandler},
    client::{Client, ClientState},
    observer::OBSERVER,
    tdlib_client::{TdJson, TdLibClient},
};
use crate::types::GetAuthorizationState;
use crate::{
    errors::{RTDError, RTDResult},
    tdjson::ClientId,
    types::{
        from_json, AuthorizationState, CheckAuthenticationCode, CheckAuthenticationPassword,
        CheckDatabaseEncryptionKey, GetApplicationConfig, RObject, RegisterUser,
        SetAuthenticationPhoneNumber, SetTdlibParameters, TdType, Update, UpdateAuthorizationState,
    },
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::{
    sync::{mpsc, RwLock},
    task::JoinHandle,
    time,
};

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

pub(crate) type StateMessage = RTDResult<ClientState, (RTDError, UpdateAuthorizationState)>;

#[derive(Debug, Clone)]
struct ClientContext<S: TdLibClient + Clone> {
    client: Client<S>,
    private_state_message_sender: mpsc::Sender<ClientState>,
    private_state_message_receiver: Arc<Mutex<mpsc::Receiver<ClientState>>>,
    pub_state_message_sender: Option<mpsc::Sender<StateMessage>>,
    pub_state_message_receiver: Option<Arc<Mutex<mpsc::Receiver<StateMessage>>>>,
}

impl<S> ClientContext<S>
where
    S: TdLibClient + Clone,
{
    pub fn client(&self) -> &Client<S> {
        &self.client
    }
    pub fn private_state_message_receiver(&self) -> &Arc<Mutex<mpsc::Receiver<ClientState>>> {
        &self.private_state_message_receiver
    }
    pub fn private_state_message_sender(&self) -> &mpsc::Sender<ClientState> {
        &self.private_state_message_sender
    }
    pub fn pub_state_message_sender(&self) -> &Option<mpsc::Sender<StateMessage>> {
        &self.pub_state_message_sender
    }
    pub fn pub_state_message_receiver(&self) -> &Option<Arc<Mutex<mpsc::Receiver<StateMessage>>>> {
        &self.pub_state_message_receiver
    }
}

type ClientsMap<S> = HashMap<ClientId, ClientContext<S>>;

/// The main object in all interactions.
/// You have to [start](crate::client::worker::Worker::start) worker and bind each client with worker using [auth_client](crate::client::worker::Worker::auth_client).
#[derive(Debug, Clone)]
pub struct Worker<A, S>
where
    A: AuthStateHandler + Send + Sync + 'static,
    S: TdLibClient + Send + Sync + Clone + 'static,
{
    run_flag: Arc<AtomicBool>,
    auth_state_handler: Arc<A>,
    read_updates_timeout: Duration,
    channels_send_timeout: Duration,
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
    /// Returns state of the client.
    pub async fn get_client_state(
        &self,
        client: &Client<T>,
    ) -> RTDResult<(ClientState, AuthorizationState)> {
        let state = client
            .get_authorization_state(GetAuthorizationState::builder().build())
            .await?;
        match &state {
            AuthorizationState::_Default => {
                panic!()
            }
            AuthorizationState::Closed(_) => Ok((ClientState::Closed, state)),
            AuthorizationState::Closing(_) => Ok((ClientState::Closed, state)),
            AuthorizationState::LoggingOut(_) => Ok((ClientState::Closed, state)),
            AuthorizationState::Ready(_) => Ok((ClientState::Opened, state)),
            AuthorizationState::WaitCode(_) => Ok((ClientState::Authorizing, state)),
            AuthorizationState::WaitEncryptionKey(_) => Ok((ClientState::Authorizing, state)),
            AuthorizationState::WaitOtherDeviceConfirmation(_) => {
                Ok((ClientState::Authorizing, state))
            }
            AuthorizationState::WaitPassword(_) => Ok((ClientState::Authorizing, state)),
            AuthorizationState::WaitPhoneNumber(_) => Ok((ClientState::Authorizing, state)),
            AuthorizationState::WaitRegistration(_) => Ok((ClientState::Authorizing, state)),
            AuthorizationState::WaitTdlibParameters(_) => Ok((ClientState::Authorizing, state)),
            AuthorizationState::GetAuthorizationState(_) => {
                panic!()
            }
        }
    }

    /// Drops authorized client.
    /// After method call you cannot interact with TDLib by the client.
    pub async fn reset_auth(&mut self, client: &mut Client<T>) -> RTDResult<()> {
        client.stop().await?;
        let client_id = client.take_client_id()?;
        self.clients.write().await.remove(&client_id);
        Ok(())
    }

    /// Method waits for client state changes.
    /// If an error occured during authorization flow, you receive [AuthorizationState](crate::types::authorization_state::AuthorizationState) on which it happened.
    /// You have to setup [channel](tokio::sync::mpsc::channel) by call [Client::builder().with_auth_state_channel(...)](Client::builder().with_auth_state_channel(...))
    pub async fn wait_auth_state_change(&self, client: &Client<T>) -> RTDResult<StateMessage> {
        let client_id = client.get_client_id()?;
        match self.clients.read().await.get(&client_id) {
            None => {Err(RTDError::BadRequest("client not authorized yet"))}
            Some(v) => {
                match v.pub_state_message_receiver() {
                    None => {Err(RTDError::BadRequest("state receiver not specified, need to call `Client::builder().with_auth_state_channel(...) before Worker::bind_client(...)"))}
                    Some(rec) => {
                        let state = rec.lock().await.recv().await.ok_or(RTDError::Internal("can't receive state: channel closed"))?;
                        Ok(state)
                    }
                }
            }
        }
    }

    /// Method waits for client state changes.
    /// It differ from [wait_auth_state_change](crate::client::worker::Worker::wait_auth_state_change) by error type: you won't receive (AuthorizationState)[crate::types::authorization_state::AuthorizationState] when error occured.
    /// Method may be useful if client already authorized on, for example, previous application startup.
    pub async fn wait_client_state(&self, client: &Client<T>) -> RTDResult<ClientState> {
        let guard = self.clients.read().await;
        match guard.get(&client.get_client_id()?) {
            None => Err(RTDError::BadRequest("client not bound yet")),
            Some(ctx) => {
                let mut rec = ctx.private_state_message_receiver().lock().await;
                Ok(rec.recv().await.unwrap())
            }
        }
    }

    /// Binds client with worker and runs authorization routines.
    /// Method returns error if worker is not running or client already bound
    pub async fn bind_client(&mut self, mut client: Client<T>) -> RTDResult<Client<T>> {
        if !self.is_running() {
            return Err(RTDError::BadRequest("worker not started yet"));
        };
        let client_id = client.get_tdlib_client().new_client();
        log::debug!("new client created: {}", client_id);
        client.set_client_id(client_id)?;

        let (sx, rx) = match client.get_auth_state_channel_size() {
            None => (None, None),
            Some(size) => {
                let (sx, rx) = mpsc::channel(size);
                (Some(sx), Some(Arc::new(Mutex::new(rx))))
            }
        };

        let (psx, prx) = mpsc::channel::<ClientState>(5);
        let ctx = ClientContext {
            client: client.clone(),
            pub_state_message_sender: sx,
            pub_state_message_receiver: rx,
            private_state_message_receiver: Arc::new(Mutex::new(prx)),
            private_state_message_sender: psx,
        };

        self.clients.write().await.insert(client_id, ctx);
        log::debug!("new client added");

        // We need to call any tdlib method to retrieve first response.
        // Otherwise client can't be authorized: no `UpdateAuthorizationState` send by TDLib.
        first_internal_request(&client.get_tdlib_client(), client_id).await;

        log::trace!("received first internal response");

        Ok(client)
    }

    /// Determines that the worker is running.
    pub fn is_running(&self) -> bool {
        self.run_flag.load(Ordering::Acquire)
    }

    #[cfg(test)]
    // Method needs for tests because we can't handle get_application_config request properly.
    pub async fn set_client(&mut self, mut client: Client<T>) -> Client<T> {
        let client_id = client.get_tdlib_client().new_client();
        log::debug!("new client created: {}", client_id);
        client.set_client_id(client_id).unwrap();

        let (psx, prx) = mpsc::channel::<ClientState>(5);
        let ctx = ClientContext {
            client: client.clone(),
            pub_state_message_sender: None,
            pub_state_message_receiver: None,
            private_state_message_receiver: Arc::new(Mutex::new(prx)),
            private_state_message_sender: psx,
        };

        self.clients.write().await.insert(client_id, ctx);
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
            tdlib_client,
            read_updates_timeout: time::Duration::from_secs_f64(read_updates_timeout),
            channels_send_timeout: time::Duration::from_secs_f64(channels_send_timeout),
            auth_state_handler: Arc::new(auth_state_handler),
            clients: Arc::new(RwLock::new(clients)),
        }
    }

    /// Starts interaction with TDLib.
    /// It returns [JoinHandle](tokio::task::JoinHandle) which allows you to handle worker state: if it yields - so worker is definitely stopped.
    pub fn start(&mut self) -> JoinHandle<()> {
        let (auth_sx, auth_rx) = mpsc::channel::<UpdateAuthorizationState>(20);

        self.run_flag.store(true, Ordering::Release);
        let updates_handle = self.init_updates_task(auth_sx);
        let auth_handle = self.init_auth_task(auth_rx);

        let run_flag = self.run_flag.clone();

        tokio::spawn(async move {
            tokio::select! {
                _ = auth_handle => {
                    log::debug!("authorization task stopped");
                },
                _ = updates_handle => {
                    log::debug!("updates task stopped");
                },
            };
            run_flag.store(false, Ordering::Release);
        })
    }

    /// Stops the client.
    /// You may want to await JoinHandle retrieved with `client.start().await` after calling `stop`.
    pub fn stop(&self) {
        self.run_flag.store(false, Ordering::Release);
    }

    // It's the base routine: sends received updates to particular handlers: observer or auth_state handler
    fn init_updates_task(&self, auth_sx: mpsc::Sender<UpdateAuthorizationState>) -> JoinHandle<()> {
        let run_flag = self.run_flag.clone();
        let clients = self.clients.clone();
        let recv_timeout = self.read_updates_timeout;
        let send_timeout = self.channels_send_timeout;
        let tdlib_client = Arc::new(self.tdlib_client.clone());

        tokio::spawn(async move {
            let current = tokio::runtime::Handle::try_current().unwrap();
            while run_flag.load(Ordering::Acquire) {
                let cl = tdlib_client.clone();
                if let Some(json) = current
                    .spawn_blocking(move || cl.receive(recv_timeout.as_secs_f64()))
                    .await
                    .unwrap()
                {
                    log::trace!("received json from tdlib: {}", json);
                    match from_json::<TdType>(&json) {
                        Err(e) => log::error!("can't deserialize tdlib data: {}", e),
                        Ok(t) => {
                            if let Some(TdType::Update(update)) = OBSERVER.notify(t) {
                                if let Update::AuthorizationState(auth_state) = update {
                                    log::trace!("auth state send: {:?}", auth_state);
                                    match auth_sx.send_timeout(auth_state, send_timeout).await {
                                        Ok(_) => {
                                            log::trace!("auth state sent");
                                        }
                                        Err(err) => {
                                            log::error!("can't send auth state update: {}", err)
                                        }
                                    };
                                } else if let Some(client_id) = update.client_id() {
                                    match clients.read().await.get(&client_id) {
                                        None => {
                                            log::warn!(
                                                "found updates for unavailable client ({})",
                                                client_id
                                            )
                                        }
                                        Some(ctx) => {
                                            if let Some(sender) = ctx.client().updates_sender() {
                                                log::trace!("sending update to client");
                                                match sender
                                                    .send_timeout(Box::new(update), send_timeout)
                                                    .await
                                                {
                                                    Ok(_) => {
                                                        log::trace!("update sent");
                                                    }
                                                    Err(err) => {
                                                        log::error!("can't send update: {}", err)
                                                    }
                                                };
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    };
                }
            }
        })
    }

    pub async fn handle_auth_state(
        &self,
        auth_state: &AuthorizationState,
        client: &Client<T>,
    ) -> RTDResult<()> {
        let clients_guard = self.clients.read().await;
        match clients_guard.get(&client.get_client_id()?) {
            None => Err(RTDError::BadRequest("client not bound yet")),
            Some(ctx) => {
                handle_auth_state(
                    client,
                    ctx.pub_state_message_sender(),
                    ctx.private_state_message_sender(),
                    self.auth_state_handler.as_ref(),
                    auth_state,
                    self.channels_send_timeout,
                )
                .await
            }
        }
    }

    // created task handles [UpdateAuthorizationState][crate::types::UpdateAuthorizationState] and sends it to particular methods of specified [AuthStateHandler](crate::client::client::AuthStateHandler)
    fn init_auth_task(
        &self,
        mut auth_rx: mpsc::Receiver<UpdateAuthorizationState>,
    ) -> JoinHandle<()> {
        let auth_state_handler = self.auth_state_handler.clone();
        let clients = self.clients.clone();
        let send_timeout = self.channels_send_timeout;

        tokio::spawn(async move {
            while let Some(auth_state) = auth_rx.recv().await {
                log::debug!("received new auth state: {:?}", auth_state);
                if let Some(client_id) = auth_state.client_id() {
                    let result = match clients.read().await.get(&client_id) {
                        None => {
                            log::warn!("found auth updates for unavailable client ({})", client_id);
                            continue;
                        }
                        Some(client_ctx) => {
                            handle_auth_state(
                                client_ctx.client(),
                                client_ctx.pub_state_message_sender(),
                                client_ctx.private_state_message_sender(),
                                auth_state_handler.as_ref(),
                                &auth_state.authorization_state(),
                                send_timeout,
                            )
                            .await
                        }
                    };

                    match result {
                        Ok(_) => {
                            log::debug!("state changes handled properly")
                        }
                        Err(err) => {
                            match clients.read().await.get(&client_id) {
                                None => {
                                    log::error!("client not found")
                                }
                                Some(cl) => {
                                    if let Some(state_sender) = cl.pub_state_message_sender() {
                                        if let Err(err) = state_sender
                                            .send_timeout(Err((err, auth_state)), send_timeout)
                                            .await
                                        {
                                            log::error!("cannot send client state changes: {}", err)
                                        }
                                    }
                                }
                            };
                        }
                    }
                }
            }
        })
    }
}

impl<A, S> Drop for Worker<A, S>
where
    A: AuthStateHandler + Send + Sync + 'static,
    S: TdLibClient + Send + Sync + Clone + 'static,
{
    fn drop(&mut self) {
        self.stop();
    }
}

async fn handle_auth_state<A: AuthStateHandler + Sync, R: TdLibClient + Clone>(
    client: &Client<R>,
    pub_state_sender: &Option<mpsc::Sender<StateMessage>>,
    private_state_sender: &mpsc::Sender<ClientState>,
    auth_state_handler: &A,
    state: &AuthorizationState,
    send_state_timeout: time::Duration,
) -> RTDResult<()> {
    log::debug!("handling new auth state: {:?}", state);
    let mut result_state = None;
    let res = match state {
        AuthorizationState::_Default => Ok(()),
        AuthorizationState::Closing(_) => Ok(()),
        AuthorizationState::LoggingOut(_) => Ok(()),
        AuthorizationState::Closed(_) => {
            result_state = Some(ClientState::Closed);
            Ok(())
        }
        AuthorizationState::Ready(_) => {
            log::debug!("ready state received, send signal");
            result_state = Some(ClientState::Opened);
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
    };

    match &result_state {
        None => {}
        Some(state) => {
            if let Err(err) = private_state_sender.send(state.clone()).await {
                {
                    log::error!(
                        "can't send state update, but state changed; error: {:?}, state: {:?}",
                        err,
                        state
                    )
                };
            }

            if let Some(sender) = &pub_state_sender {
                if let Err(err) = sender
                    .send_timeout(Ok(state.clone()), send_state_timeout)
                    .await
                {
                    log::error!(
                        "can't send state update, but state changed; error: {:?}, state: {:?}",
                        err,
                        state
                    )
                };
            }
        }
    }
    res
}

async fn first_internal_request<S: TdLibClient>(tdlib_client: &S, client_id: ClientId) {
    let req = GetApplicationConfig::builder().build();
    let extra = match req.as_ref().extra().ok_or(RTDError::Internal(
        "invalid tdlib response type, not have `extra` field",
    )) {
        Ok(v) => v,
        Err(err) => {
            log::error!("{}", err);
            return;
        }
    };
    let signal = OBSERVER.subscribe(&extra);
    if let Err(err) = tdlib_client.send(client_id, req.as_ref()) {
        log::error!("{}", err);
        return;
    };

    let received = signal.await;
    OBSERVER.unsubscribe(&extra);
    match received {
        Err(_) => log::error!("receiver already closed"),
        Ok(v) => match v {
            TdType::JsonValue(_) => {}
            TdType::Error(v) => log::error!("{}", v.message()),
            _ => {
                log::error!("invalid response received: {:?}", v);
            }
        },
    };
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
            worker.bind_client(
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
            worker.bind_client(
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
