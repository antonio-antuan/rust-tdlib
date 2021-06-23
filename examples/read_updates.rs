use rust_tdlib::client::ClientState;
use rust_tdlib::{
    client::{Client, Worker},
    tdjson,
    types::{TdlibParameters, Update},
};

#[tokio::main]
async fn main() {
    tdjson::set_log_verbosity_level(1);
    env_logger::init();
    let tdlib_parameters = TdlibParameters::builder()
        .database_directory("tddb")
        .use_test_dc(false)
        .api_id(env!("API_ID").parse::<i32>().unwrap())
        .api_hash(env!("API_HASH"))
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
        let mut wait_messages: i32 = 100;
        while let Some(message) = receiver.recv().await {
            log::info!("updates handler received {:?}", message);
            wait_messages -= 1;
            if wait_messages == 0 {
                break;
            }
        }
    });

    let mut worker = Worker::builder().build().unwrap();
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
