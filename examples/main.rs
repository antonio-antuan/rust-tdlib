use rust_tdlib::types::{CreatePrivateChat, GetUser, SearchPublicChat};
use rust_tdlib::{
    client::{Client, Worker},
    tdjson,
    types::{
        FormattedText, GetMe, InputMessageContent, InputMessageText, JoinChat, SearchPublicChats,
        SendMessage, TdlibParameters, Update,
    },
};
use std::borrow::Borrow;

#[tokio::main]
async fn main() {
    tdjson::set_log_verbosity_level(1);
    env_logger::init();
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<Box<Update>>(10000);

    let client1 = Client::builder()
        .with_tdlib_parameters(
            TdlibParameters::builder()
                .database_directory("tddb1")
                .use_test_dc(false)
                .api_id(env!("API_ID").parse::<i32>().unwrap())
                .api_hash(env!("API_HASH"))
                .system_language_code("en")
                .device_model("Desktop")
                .system_version("Unknown")
                .application_version(env!("CARGO_PKG_VERSION"))
                .enable_storage_optimizer(true)
                .build(),
        )
        .with_updates_sender(sender)
        .build()
        .unwrap();
    let client2 = Client::builder()
        .with_tdlib_parameters(
            TdlibParameters::builder()
                .database_directory("tddb2")
                .use_test_dc(false)
                .api_id(env!("API_ID").parse::<i32>().unwrap())
                .api_hash(env!("API_HASH"))
                .system_language_code("en")
                .device_model("Desktop")
                .system_version("Unknown")
                .application_version(env!("CARGO_PKG_VERSION"))
                .enable_storage_optimizer(true)
                .build(),
        )
        .build()
        .unwrap();

    let mut worker = Worker::builder().build().unwrap();

    let waiter = worker.start();

    let (client1_state, client1) = worker.auth_client(client1).await.unwrap();
    log::info!("client1 authorized");
    let me1 = client1.get_me(GetMe::builder().build()).await.unwrap();
    log::info!("me 1: {:?}", me1);

    let (client2_state, client2) = worker.auth_client(client2).await.unwrap();
    log::info!("client2 authorized");
    let me2 = client2.get_me(GetMe::builder().build()).await.unwrap();
    log::info!("me 2: {:?}", me2);

    let chats = client1
        .search_public_chats(SearchPublicChats::builder().query("@rust").build())
        .await
        .unwrap();
    log::info!("found chats with client1: {}", chats.chat_ids().len());

    let chats = client2
        .search_public_chats(SearchPublicChats::builder().query("@telegram").build())
        .await
        .unwrap();
    log::info!("found chats with client2: {}", chats.chat_ids().len());

    log::info!("get client");
    let chat_with_yourself = client1
        .search_public_chat(SearchPublicChat::builder().username(me1.username()).build())
        .await
        .unwrap();

    client1
        .send_message(
            SendMessage::builder()
                .chat_id(chat_with_yourself.id())
                .input_message_content(InputMessageContent::InputMessageText(
                    InputMessageText::builder()
                        .text(FormattedText::builder().text("hello").build())
                        .build(),
                ))
                .build(),
        )
        .await
        .unwrap();
    let mut wait_messages: i32 = 100;
    while let Some(message) = receiver.recv().await {
        match message.borrow() {
            Update::NewMessage(new_message) => {
                log::info!("new message received: {:?}", new_message)
            }
            _ => {}
        }
        wait_messages -= 1;
        if wait_messages == 0 {
            break;
        }
    }
    client1.stop().await.unwrap();
    client2.stop().await.unwrap();

    client1_state.await.unwrap();
    log::info!("client1 closed");

    client2_state.await.unwrap();
    log::info!("client2 closed");

    worker.stop();
    log::info!("worker stopped");

    waiter.await.unwrap();
    log::info!("waiter stopped");
}
