use rust_tdlib::client::SignalAuthStateHandler;
use rust_tdlib::{
    client::{Client, Worker},
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
        .build()
        .unwrap();

    tokio::spawn(async move {
        let mut wait_messages: i32 = 1000;
        while let Some(message) = receiver.recv().await {
            log::info!("updates handler received {:?}", message);
            wait_messages -= 1;
            if wait_messages == 0 {
                break;
            }
        }
    });

    let (sx, rx) = tokio::sync::mpsc::channel(5);
    let auth_handler = SignalAuthStateHandler::new(rx);
    sx.send("".to_string()).await.unwrap(); // empty encryption key
    let client_type = std::env::var("CLIENT_TYPE").unwrap_or_else(|_| "b".to_string());
    match client_type.as_str() {
        "p" | "b" => {}
        _ => {
            panic!(
                "expect one of \"p\" or \"b\" for CLIENT_TYPE env variable, got {}",
                client_type
            )
        }
    }
    sx.send(client_type).await.unwrap(); // bot
    sx.send(std::env::var("BOT_TOKEN").unwrap()).await.unwrap(); // empty encryption key

    let mut worker = Worker::builder()
        .with_auth_state_handler(auth_handler)
        .build()
        .unwrap();
    worker.start();

    worker.bind_client(client).await.unwrap();

    tokio::time::sleep(Duration::from_secs(60)).await;
    log::info!("stop worker");
    worker.stop();
}
