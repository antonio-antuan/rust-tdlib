use std::io;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::types::{
    AuthorizationStateWaitCode, AuthorizationStateWaitEncryptionKey,
    AuthorizationStateWaitOtherDeviceConfirmation, AuthorizationStateWaitPassword,
    AuthorizationStateWaitPhoneNumber, AuthorizationStateWaitRegistration,
};

/// `ClientIdentifier` allows to determine if client is bot (with bot token as identifier) or client (with a phone number)
#[derive(Debug, Clone)]
pub enum ClientIdentifier {
    PhoneNumber(String),
    BotToken(String),
}

/// `AuthStateHandler` trait provides methods that returns data, required for authentication
/// It allows you to handle particular "auth states", such as [WaitPassword](crate::types::AuthorizationStateWaitPassword), [WaitPhoneNumber](crate::types::AuthorizationStateWaitPhoneNumber) and so on.
#[async_trait]
pub trait AuthStateHandler {
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

    fn wait_input(&self) -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(e) => panic!("Can not get input value: {:?}", e),
        }
    }
}

#[async_trait]
impl AuthStateHandler for ConsoleAuthStateHandler {
    async fn handle_wait_code(&self, _wait_code: &AuthorizationStateWaitCode) -> String {
        println!("waiting for auth code");
        self.wait_input()
    }

    async fn handle_encryption_key(
        &self,
        _wait_encryption_key: &AuthorizationStateWaitEncryptionKey,
    ) -> String {
        println!("waiting for encryption key");
        self.wait_input()
    }

    async fn handle_wait_password(
        &self,
        _wait_password: &AuthorizationStateWaitPassword,
    ) -> String {
        println!("waiting for password");
        self.wait_input()
    }

    async fn handle_wait_client_identifier(
        &self,
        _: &AuthorizationStateWaitPhoneNumber,
    ) -> ClientIdentifier {
        loop {
            println!("choose one of phone number (p) or bot token (b)");
            let inp = self.wait_input();
            match inp.to_lowercase().trim() {
                "b" => {
                    println!("enter bot token");
                    return ClientIdentifier::BotToken(self.wait_input());
                }
                "p" => {
                    println!("enter phone number");
                    return ClientIdentifier::PhoneNumber(self.wait_input());
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
        _wait_registration: &AuthorizationStateWaitRegistration,
    ) -> (String, String) {
        loop {
            println!("waiting for first_name and second_name separated by comma");
            let inp: String = self.wait_input();
            if let Some((f, l)) = split_string(inp, ',') {
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
    async fn handle_wait_code(&self, _: &AuthorizationStateWaitCode) -> String {
        log::info!("waiting for auth code");
        self.wait_signal().await
    }

    async fn handle_encryption_key(&self, _: &AuthorizationStateWaitEncryptionKey) -> String {
        log::info!("waiting for encryption key");
        let f = self.wait_signal().await;
        log::info!("get encryption key");
        f
    }

    async fn handle_wait_password(&self, _: &AuthorizationStateWaitPassword) -> String {
        log::info!("waiting for password");
        self.wait_signal().await
    }

    async fn handle_wait_client_identifier(
        &self,
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
        _: &AuthorizationStateWaitRegistration,
    ) -> (String, String) {
        loop {
            log::info!("waiting for first name and last name separated by comma");
            let inp = self.wait_signal().await;
            if let Some((f, l)) = split_string(inp, ',') {
                return (f, l);
            }
        }
    }
}

fn split_string(input: String, sep: char) -> Option<(String, String)> {
    let found: Vec<&str> = input.splitn(2, |c| c == sep).collect();
    if let 2 = found.len() {
        let f = found.get(0).unwrap().trim();
        let s = found.get(1).unwrap().trim();
        if !f.is_empty() && !s.is_empty() {
            return Some((f.to_string(), s.to_string()));
        }
    }
    None
}
