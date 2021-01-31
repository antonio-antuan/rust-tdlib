#[macro_use]
extern crate log;

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
    let mut client = Client::builder()
        .with_tdlib_verbosity_level(env!("TD_LOG_LEVEL").parse().unwrap())
        .with_tdlib_log_file_path("log")
        .with_tdlib_parameters(tdlib_parameters)
        .build()
        .unwrap();

    let waiter = client.start().await.unwrap();
    let api = client.api();

    let me = api.get_me(GetMe::builder().build()).await.unwrap();
    info!("me: {:?}", me);

    let chats = api
        .search_public_chats(SearchPublicChats::builder().query("@rust").build())
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
    client.stop();
    waiter.await.unwrap();
    info!("client closed");
}
