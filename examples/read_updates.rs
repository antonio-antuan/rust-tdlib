use log;
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
        .database_directory("tdlib")
        .use_test_dc(false)
        .api_id(env!("API_ID").parse::<i32>().unwrap())
        .api_hash(env!("API_HASH"))
        .system_language_code("en")
        .device_model("Desktop")
        .system_version("Unknown")
        .application_version(env!("CARGO_PKG_VERSION"))
        .enable_storage_optimizer(true)
        .build();

    let (sender, mut receiver) = tokio::sync::mpsc::channel::<Update>(100);

    let client = Client::builder()
        .with_tdlib_parameters(tdlib_parameters)
        .with_updates_sender(sender)
        .build()
        .unwrap();

    let mut reader_waiter = tokio::spawn(async move {
        // we need to specify large number because first updates always sent by tdlib in the beginning of process
        let mut wait_messages: i32 = 40;
        while let Some(message) = receiver.recv().await {
            log::info!("updates handler received {:?}", message);
            wait_messages -= 1;
            if wait_messages == 0 {
                break;
            }
        };
    });

    let mut worker = Worker::builder().build().unwrap();
    let mut waiter = worker.start();

    let v = tokio::select! {
        c = worker.auth_client(client) => {match c {
            Ok((s, cl)) => Some((s, cl)),
            Err(e) => panic!("{:?}", e)
        }}
        w = &mut waiter => panic!("{:?}", w),
        _ = &mut reader_waiter => {
            log::info!("reader closed");
            None
        },
    };

    match v {
        Some((_, client)) => {
            client.stop().await.unwrap();
            reader_waiter.await.unwrap();
        },
        None => {
            worker.stop();
        }
    }

    waiter.await.unwrap();


}
