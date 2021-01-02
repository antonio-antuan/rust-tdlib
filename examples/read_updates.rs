#[macro_use]
extern crate log;

use rust_tdlib::{client::Client, types::*};

#[tokio::main]
async fn main() {
    env_logger::init();
    let tdlib_parameters = TdlibParameters::builder()
        .database_directory("tdlib")
        .use_test_dc(false)
        .api_id(env!("API_ID").parse::<i64>().unwrap())
        .api_hash(env!("API_HASH"))
        .enable_storage_optimizer(true)
        .build();
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<TdType>(10);

    let mut client = Client::builder()
        .with_tdlib_parameters(tdlib_parameters)
        .with_updates_sender(sender)
        .build()
        .unwrap();

    let waiter = client.start().await.unwrap();
    let mut wait_messages_num = 1;

    while wait_messages_num > 0 {
        let message = receiver.recv().await.unwrap();
        info!("updates handler received {:?}", message);
        wait_messages_num -= wait_messages_num;
    }

    client.stop();
    waiter.await.unwrap();
    info!("client closed");
}
