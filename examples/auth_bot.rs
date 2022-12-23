use rust_tdlib::client::{ClientState, SignalAuthStateHandler};
use rust_tdlib::{
    client::{Client, Worker},
    tdjson,
    types::{TdlibParameters, Update},
};

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

    let mut reader_waiter = tokio::spawn(async move {
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
    sx.send("p".to_string()).await.unwrap(); // bot
    sx.send(std::env::var("BOT_TOKEN").unwrap()).await.unwrap(); // empty encryption key

    let mut worker = Worker::builder()
        .with_auth_state_handler(auth_handler)
        .build()
        .unwrap();
    let mut waiter = worker.start();

    let v = tokio::select! {
        c = worker.bind_client(client) => {
            match c {
                Ok(cl) => Some(cl),
                Err(e) => panic!("{:?}", e)
            }
        }
        w = &mut waiter => panic!("{:?}", w),
        _ = &mut reader_waiter => {
            log::info!("reader closed");
            None
        },
    };

    if let Some(client) = v {
        log::info!("stop client");
        client.stop().await.unwrap();
        log::info!("client stopped");
        loop {
            if worker.wait_client_state(&client).await.unwrap() == ClientState::Closed {
                log::info!("closed");
                break;
            }
        }
    }

    log::info!("stop worker");
    worker.stop();
}
