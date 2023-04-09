use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use dyn_clone::DynClone;
use tokio::sync::Mutex;

use crate::types::{
    AuthorizationStateWaitCode, AuthorizationStateWaitEncryptionKey,
    AuthorizationStateWaitOtherDeviceConfirmation, AuthorizationStateWaitPassword,
    AuthorizationStateWaitPhoneNumber, AuthorizationStateWaitRegistration,
};

use crate::utils;

/// `ClientIdentifier` allows to determine if client is bot (with bot token as identifier) or client (with a phone number)
#[derive(Debug, Clone)]
pub enum ClientIdentifier {
    PhoneNumber(String),
    BotToken(String),
}

#[async_trait]
pub trait ClientAuthStateHandler: DynClone + Send + Sync + Debug {
    /// Interacts with provided link
    async fn handle_other_device_confirmation(
        &self,
        wait_device_confirmation: &AuthorizationStateWaitOtherDeviceConfirmation,
    ) {
        println!(
            "other device confirmation link: {}",
            wait_device_confirmation.link()
        );
    }
    /// Returns wait code
    async fn handle_wait_code(&self, wait_code: &AuthorizationStateWaitCode) -> String;
    /// Returns database encryption key
    async fn handle_encryption_key(
        &self,
        wait_encryption_key: &AuthorizationStateWaitEncryptionKey,
    ) -> String;
    /// Returns password
    async fn handle_wait_password(&self, wait_password: &AuthorizationStateWaitPassword) -> String;
    /// Returns [ClientIdentifier](crate::client::auth_handler::ClientIdentifier)
    async fn handle_wait_client_identifier(
        &self,
        wait_phone_number: &AuthorizationStateWaitPhoneNumber,
    ) -> ClientIdentifier;
    /// Returns first_name and second_name
    async fn handle_wait_registration(
        &self,
        wait_registration: &AuthorizationStateWaitRegistration,
    ) -> (String, String);
}

dyn_clone::clone_trait_object!(ClientAuthStateHandler);

/// `AuthStateHandler` trait provides methods that returns data, required for authentication
/// It allows you to handle particular "auth states", such as [WaitPassword](crate::types::AuthorizationStateWaitPassword), [WaitPhoneNumber](crate::types::AuthorizationStateWaitPhoneNumber) and so on.
#[async_trait]
pub trait AuthStateHandler {
    /// Interacts with provided link
    async fn handle_other_device_confirmation(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        wait_device_confirmation: &AuthorizationStateWaitOtherDeviceConfirmation,
    ) {
        println!(
            "other device confirmation link: {}",
            wait_device_confirmation.link()
        );
    }
    /// Returns wait code
    async fn handle_wait_code(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_code: &AuthorizationStateWaitCode,
    ) -> String;
    /// Returns database encryption key
    async fn handle_encryption_key(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_encryption_key: &AuthorizationStateWaitEncryptionKey,
    ) -> String;
    /// Returns password
    async fn handle_wait_password(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_password: &AuthorizationStateWaitPassword,
    ) -> String;
    /// Returns [ClientIdentifier](crate::client::auth_handler::ClientIdentifier)
    async fn handle_wait_client_identifier(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_phone_number: &AuthorizationStateWaitPhoneNumber,
    ) -> ClientIdentifier;
    /// Returns first_name and second_name
    async fn handle_wait_registration(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_registration: &AuthorizationStateWaitRegistration,
    ) -> (String, String);
}

/// Provides minimal implementation of `AuthStateHandler`.
/// All required methods wait (synchronously) for stdin input
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
}

#[async_trait]
impl AuthStateHandler for ConsoleAuthStateHandler {
    async fn handle_wait_code(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        _wait_code: &AuthorizationStateWaitCode,
    ) -> String {
        println!("waiting for auth code");
        utils::wait_input_sync()
    }

    async fn handle_encryption_key(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        _wait_encryption_key: &AuthorizationStateWaitEncryptionKey,
    ) -> String {
        println!("waiting for encryption key");
        utils::wait_input_sync()
    }

    async fn handle_wait_password(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        _wait_password: &AuthorizationStateWaitPassword,
    ) -> String {
        println!("waiting for password");
        utils::wait_input_sync()
    }

    async fn handle_wait_client_identifier(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        _: &AuthorizationStateWaitPhoneNumber,
    ) -> ClientIdentifier {
        loop {
            println!("choose one of phone number (p) or bot token (b)");
            let inp = utils::wait_input_sync();
            match inp.to_lowercase().trim() {
                "b" => {
                    println!("enter bot token");
                    return ClientIdentifier::BotToken(utils::wait_input_sync());
                }
                "p" => {
                    println!("enter phone number");
                    return ClientIdentifier::PhoneNumber(utils::wait_input_sync());
                }
                _ => {
                    // invalid input, next iteration}
                    continue;
                }
            }
        }
    }
    async fn handle_wait_registration(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        _wait_registration: &AuthorizationStateWaitRegistration,
    ) -> (String, String) {
        loop {
            println!("waiting for first_name and second_name separated by comma");
            let inp: String = utils::wait_input_sync();
            if let Some((f, l)) = utils::split_string(inp, ',') {
                return (f, l);
            }
        }
    }
}

/// All required methods wait for data sent by [Sender](tokio::sync::mpsc::Sender).
#[derive(Debug, Clone)]
pub struct SignalAuthStateHandler {
    rec: Arc<Mutex<tokio::sync::mpsc::Receiver<String>>>,
}

impl SignalAuthStateHandler {
    pub fn new(receiver: tokio::sync::mpsc::Receiver<String>) -> Self {
        Self {
            rec: Arc::new(Mutex::new(receiver)),
        }
    }

    async fn wait_signal(&self) -> String {
        let mut guard = self.rec.lock().await;
        guard.recv().await.expect("no signals received")
    }
}

#[async_trait]
impl AuthStateHandler for SignalAuthStateHandler {
    async fn handle_wait_code(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        _: &AuthorizationStateWaitCode,
    ) -> String {
        log::info!("waiting for auth code");
        self.wait_signal().await
    }

    async fn handle_encryption_key(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        _: &AuthorizationStateWaitEncryptionKey,
    ) -> String {
        log::info!("waiting for encryption key");
        let f = self.wait_signal().await;
        log::info!("get encryption key");
        f
    }

    async fn handle_wait_password(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        _: &AuthorizationStateWaitPassword,
    ) -> String {
        log::info!("waiting for password");
        self.wait_signal().await
    }

    async fn handle_wait_client_identifier(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        _: &AuthorizationStateWaitPhoneNumber,
    ) -> ClientIdentifier {
        loop {
            log::info!("choose one of phone number (p) or bot token (b)");
            let inp = self.wait_signal().await;
            match inp.to_lowercase().trim() {
                "b" => {
                    log::info!("enter bot token");
                    return ClientIdentifier::BotToken(self.wait_signal().await);
                }
                "p" => {
                    log::info!("enter phone number");
                    return ClientIdentifier::PhoneNumber(self.wait_signal().await);
                }
                _ => {
                    // invalid input, next iteration}
                    continue;
                }
            }
        }
    }

    async fn handle_wait_registration(
        &self,
        _client: Box<dyn ClientAuthStateHandler>,
        _: &AuthorizationStateWaitRegistration,
    ) -> (String, String) {
        loop {
            log::info!("waiting for first name and last name separated by comma");
            let inp = self.wait_signal().await;
            if let Some((f, l)) = utils::split_string(inp, ',') {
                return (f, l);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthStateHandlerProxy(Option<String>);

impl AuthStateHandlerProxy {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn new_with_encryption_key(key: String) -> Self {
        Self(Some(key))
    }
}

#[async_trait]
impl AuthStateHandler for AuthStateHandlerProxy {
    async fn handle_other_device_confirmation(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_device_confirmation: &AuthorizationStateWaitOtherDeviceConfirmation,
    ) {
        client
            .handle_other_device_confirmation(wait_device_confirmation)
            .await
    }

    async fn handle_wait_code(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_code: &AuthorizationStateWaitCode,
    ) -> String {
        client.handle_wait_code(wait_code).await
    }

    async fn handle_encryption_key(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_encryption_key: &AuthorizationStateWaitEncryptionKey,
    ) -> String {
        match &self.0 {
            None => {
                log::info!("wait for client's encryption key");
                client.handle_encryption_key(wait_encryption_key).await
            },
            Some(key) => {
                key.clone()
            }
        }
    }

    async fn handle_wait_password(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_password: &AuthorizationStateWaitPassword,
    ) -> String {
        client.handle_wait_password(wait_password).await
    }

    async fn handle_wait_client_identifier(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_phone_number: &AuthorizationStateWaitPhoneNumber,
    ) -> ClientIdentifier {
        client
            .handle_wait_client_identifier(wait_phone_number)
            .await
    }

    async fn handle_wait_registration(
        &self,
        client: Box<dyn ClientAuthStateHandler>,
        wait_registration: &AuthorizationStateWaitRegistration,
    ) -> (String, String) {
        client.handle_wait_registration(wait_registration).await
    }
}
