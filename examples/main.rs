#[macro_use]
extern crate log;

use env_logger;
use rtdlib_sys::Tdlib;
use std::io;
use std::sync::Arc;
use std::sync::{Condvar, Mutex};
use telegram_tdlib::{
    client::api::Api, client::client::TypeInAuthStateHandler, client::Client, types::*,
};
use tokio::sync::mpsc::Receiver;

#[tokio::main]
async fn main() {
    env_logger::init();
    let tdlib_parameters = TdlibParameters::builder()
        .database_directory("tdlib")
        .use_test_dc(false)
        .api_id(env!("API_ID").parse::<i64>().unwrap())
        .api_hash(env!("API_HASH"))
        .system_language_code("en")
        .device_model("Desktop")
        .system_version("Unknown")
        .application_version(env!("CARGO_PKG_VERSION"))
        .enable_storage_optimizer(true)
        .build();
    let mut client = Client::new(Tdlib::new(), TypeInAuthStateHandler {}, tdlib_parameters);
    client.set_log_verbosity_level(2);

    let (sender, mut receiver) = tokio::sync::mpsc::channel::<TdType>(100);
    client.set_updates_sender(sender);
    client.start().await.unwrap();
    let api = client.api();
    let updates_join = tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            info!("updates handler received {:?}", message);
        }
    });
    let chats = api
        .search_public_chats(SearchPublicChats::builder().query("rust async").build())
        .await
        .unwrap();
    info!("found chats: {}", chats.chat_ids().len());
    for chat_id in chats.chat_ids() {
        let chat = api
            .get_chat(GetChat::builder().chat_id(*chat_id))
            .await
            .unwrap();
        info!("{:?}", chat)
    }
}

async fn authorization_routine(
    api: &Api,
    update_auth_state: UpdateAuthorizationState,
    have_authorization: Arc<(Mutex<bool>, Condvar)>,
) {
    match update_auth_state.authorization_state() {
        AuthorizationState::_Default(_) => {}
        AuthorizationState::Closed(_) => {}
        AuthorizationState::Closing(_) => {}
        AuthorizationState::LoggingOut(_) => {}
        AuthorizationState::Ready(_) => {
            info!("ready");
            let (lock, cvar) = &*have_authorization;
            let mut authorized = lock.lock().unwrap();
            *authorized = true;
            cvar.notify_one();
        }
        AuthorizationState::WaitCode(_) => {
            info!("wait for auth code");
            let code = type_in();
            let code = CheckAuthenticationCode::builder().code(&code).build();
            api.check_authentication_code(&code).await.unwrap();
            info!("Set auth code");
        }
        AuthorizationState::WaitEncryptionKey(_) => {
            info!("going to set encryption key");
            let params = SetDatabaseEncryptionKey::builder().build();
            api.set_database_encryption_key(&params).await.unwrap();
            info!("Set encryption key");
        }
        AuthorizationState::WaitOtherDeviceConfirmation(_) => {}
        AuthorizationState::WaitPassword(_) => {}
        AuthorizationState::WaitPhoneNumber(_) => {
            info!("going to set phone number");
            let params = SetAuthenticationPhoneNumber::builder()
                .phone_number(env!("TG_PHONE"))
                .build();
            api.set_authentication_phone_number(&params).await.unwrap();
            info!("Set phone number");
        }
        AuthorizationState::WaitRegistration(_) => {}
        AuthorizationState::WaitTdlibParameters(_) => {
            let params = SetTdlibParameters::builder()
                .parameters(
                    TdlibParameters::builder()
                        .database_directory("tdlib")
                        .use_test_dc(false)
                        .api_id(env!("API_ID").parse::<i64>().unwrap())
                        .api_hash(env!("API_HASH"))
                        .system_language_code("en")
                        .device_model("Desktop")
                        .system_version("Unknown")
                        .application_version(env!("CARGO_PKG_VERSION"))
                        .enable_storage_optimizer(true)
                        .build(),
                )
                .build();
            api.set_tdlib_parameters(&params).await.unwrap();
        }
        AuthorizationState::GetAuthorizationState(_) => {}
    };
}

fn type_in() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => panic!("Can not get input value: {:?}", e),
    }
}
