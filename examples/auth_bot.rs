use rust_tdlib::client::{AuthStateHandlerProxy, ClientIdentifier};
use rust_tdlib::{
    client::{Client, ConsoleClientStateHandlerIdentified, Worker},
    tdjson,
    types::{TdlibParameters, Update},
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    tdjson::set_log_verbosity_level(
        std::env::var("TDLIB_LOG_VERBOSITY")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .unwrap(),
    );
    env_logger::init();
    let tdlib_parameters = TdlibParameters::builder()
        .database_directory("tddb")
        .use_test_dc(false)
        .api_id(std::env::var("API_ID").unwrap().parse::<i32>().unwrap())
        .api_hash(std::env::var("API_HASH").unwrap())
        .system_language_code("en")
        .device_model("Desktop")
        .system_version("Unknown")
        .application_version(env!("CARGO_PKG_VERSION"))
        .enable_storage_optimizer(true)
        .build();

    let (sender, mut receiver) = tokio::sync::mpsc::channel::<Box<Update>>(100);

    let client = Client::builder()
        .with_tdlib_parameters(tdlib_parameters)
        .with_updates_sender(sender)
        .with_client_auth_state_handler(ConsoleClientStateHandlerIdentified::new(
            ClientIdentifier::PhoneNumber(std::env::var("PHONE_NUMBER").unwrap()),
        ))
        .build()
        .unwrap();

    tokio::spawn(async move {
        let mut wait_messages: i32 = 2000;
        while let Some(message) = receiver.recv().await {
            log::info!("updates handler received {:?}", message);
            wait_messages -= 1;
            if wait_messages == 0 {
                break;
            }
        }
    });

    let mut worker = Worker::builder()
        .with_auth_state_handler(AuthStateHandlerProxy::default())
        .build()
        .unwrap();
    worker.start();

    let client = worker.bind_client(client).await.unwrap();

    tokio::time::sleep(Duration::from_secs(30)).await;

    worker.reload_client(client).await.unwrap();

    log::info!("client was reloaded");

    // check that updates still received
    tokio::time::sleep(Duration::from_secs(30)).await;
    log::info!("stop worker");

    worker.stop();
}
