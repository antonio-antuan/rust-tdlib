use rust_tdlib::{
    client::{Client, Worker, SignalAuthStateHandler},
    tdjson,
    types::{TdlibParameters},
};
use std::io::Error;
use rust_tdlib::client::client::{ClientState, AuthError};
use rust_tdlib::types::SetAuthenticationPhoneNumber;

#[tokio::main]
async fn main() {
    tdjson::set_log_verbosity_level(1);
    env_logger::init();
    
    let db_dir = "tddb_auth";
    
    if std::path::Path::new(db_dir).exists() {
        // we need to check auth errors and so on so we need to ensure that authorization not done yet
        std::fs::remove_dir_all(db_dir).expect("can't clear directory");
    }
    
    let tdlib_parameters = TdlibParameters::builder()
        .database_directory(db_dir)
        .use_test_dc(false)
        .api_id(env!("API_ID").parse::<i32>().unwrap())
        .api_hash(env!("API_HASH"))
        .system_language_code("en")
        .device_model("Desktop")
        .system_version("Unknown")
        .application_version(env!("CARGO_PKG_VERSION"))
        .enable_storage_optimizer(true)
        .build();

    let mut client = Client::builder()
        .with_tdlib_parameters(tdlib_parameters)
        .build()
        .unwrap();

    let (sx, rx) = tokio::sync::mpsc::channel(5);
    let auth_handler = SignalAuthStateHandler::new(rx);
    let mut worker = Worker::builder().with_auth_state_handler(auth_handler).build().unwrap();
    let mut waiter = worker.start();

    sx.send("".to_string()).await; // empty encryption key
    sx.send("invalid phone".to_string()).await; // wrong phone number: not a number


    let client = worker.bind_client(client).await.expect("can't bind client");


    match worker.wait_auth_state_change(&client).await {
        Ok(s) => {match s {
            ClientState::Opened => {}
            ClientState::Closed => {}
            ClientState::Authorizing => {}
            ClientState::Error(err) => {match err {
                AuthError::PhoneError(phone_err) => {
                    client.set_authentication_phone_number(SetAuthenticationPhoneNumber::builder().phone_number(env!("PHONE_NUMBER")).build()).await.expect("can't set phone number")
                }
                AuthError::CodeError(_) => {log::error!("received code error, need to handle it here")}
                _ => log::error!("{:?}", err)
            }}
        }}
        Err(err) => {log::error!("auth error: {}", err)}
    }

}