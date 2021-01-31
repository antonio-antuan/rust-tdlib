#[macro_use]
extern crate log;
use futures::{future::FutureExt};
use rust_tdlib::{client::Client, types::*};

#[tokio::main]
async fn main() {
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
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<TdType>(10);

    let mut client = Client::builder()
        .with_tdlib_parameters(tdlib_parameters)
        .with_tdlib_verbosity_level(env!("TD_LOG_LEVEL").parse().unwrap())
        // .with_tdlib_log_file_path("tdlogs")
        .with_updates_sender(sender)
        .build()
        .unwrap();

    let cl_waiter = client.start().await.unwrap();


    let reader_waiter = tokio::spawn(async move {

        let mut wait_messages_num: i32 = 1;
        while wait_messages_num > 0 {
            let message = receiver.recv().await.unwrap();
            info!("updates handler received {:?}", message);
            wait_messages_num -= wait_messages_num;
        }
    });
    let me = client.api().get_me(GetMe::builder().build()).await.unwrap();
    info!("me: {:?}", me);
    tokio::select! {
        h = cl_waiter => panic!("{:?}", h),
        w = reader_waiter =>  panic!("{:?}", w)
    }

    client.stop();
    cl_waiter.await.unwrap();
    info!("client closed");
}
