#[macro_use]
extern crate log;

use rust_tdlib::{
    client::{Client, Worker},
    types::*,
    tdjson
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
    let client = Client::builder()
        .with_tdlib_parameters(tdlib_parameters.clone())
        .build()
        .unwrap();

    let mut worker = Worker::builder().build().unwrap();

    let waiter = worker.start();
    let (client_state, client) = worker.auth_client(client).await.unwrap();

    let me = client.get_me(GetMe::builder().build()).await.unwrap();
    info!("me: {:?}", me);

    let chats = client
        .search_public_chats(SearchPublicChats::builder().query("@rust").build())
        .await
        .unwrap();

    info!("found chats: {}", chats.chat_ids().len());
    for chat_id in chats.chat_ids() {
        let chat = client
            .get_chat(GetChat::builder().chat_id(*chat_id))
            .await
            .unwrap();
        info!("{:?}", chat)
    }
    worker.stop();
    info!("worker stopped");
    client_state.await.unwrap();
    info!("client closed");
    waiter.await.unwrap();
    info!("worker stopped");
}
