#[macro_use]
extern crate log;

use env_logger;
use rust_tdlib::{
    client::{Client, RawApi, Tdlib, TypeInAuthStateHandler},
    types::*,
};

#[tokio::main]
async fn main() {
    Tdlib::set_log_verbosity_level(1);
    env_logger::init();
    let tdlib_parameters = TdlibParameters::builder()
        .database_directory("tdlib")
        .use_test_dc(false)
        .api_id(env!("API_ID").parse::<i64>().unwrap())
        .api_hash(env!("API_HASH"))
        .system_language_code("en")
        .device_model("Desktop")
        .system_version("Unknown")
        .application_version(env!("CARGO_PKG_VERSION"))
        .enable_storage_optimizer(true)
        .build();
    let mut client = Client::new(
        RawApi::default(),
        TypeInAuthStateHandler {},
        tdlib_parameters,
    );

    let (sender, mut receiver) = tokio::sync::mpsc::channel::<TdType>(100);
    client.set_updates_sender(sender);
    client.start().await.unwrap();
    let api = client.api();

    let _updates_join = tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            info!("updates handler received {:?}", message);
        }
    });
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
}
