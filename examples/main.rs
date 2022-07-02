use rust_tdlib::types::MessageContent;
use rust_tdlib::{
    client::{Client, ClientState, Worker},
    tdjson,
    types::{
        FormattedText, GetMe, InputMessageContent, InputMessageText, SearchPublicChat,
        SearchPublicChats, SendMessage, TdlibParameters, Update,
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
                .api_id(std::env::var("API_ID").unwrap().parse::<i32>().unwrap())
                .api_hash(std::env::var("API_HASH").unwrap())
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
                .api_id(std::env::var("API_ID").unwrap().parse::<i32>().unwrap())
                .api_hash(std::env::var("API_HASH").unwrap())
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

    worker.start();

    let client1 = worker.bind_client(client1).await.unwrap();

    loop {
        if worker.wait_client_state(&client1).await.unwrap() == ClientState::Opened {
            log::info!("client1 authorized");
            break;
        }
    }
    let me1 = client1.get_me(GetMe::builder().build()).await.unwrap();
    log::info!("me 1: {:?}", me1);

    let client2 = worker.bind_client(client2).await.unwrap();
    loop {
        if worker.wait_client_state(&client2).await.unwrap() == ClientState::Opened {
            log::info!("client2 authorized");
            break;
        }
    }
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

    let text_to_send = format!("hello from {}", me1.username());
    log::info!("sending text: {}", text_to_send);

    client1
        .send_message(
            SendMessage::builder()
                .chat_id(chat_with_yourself.id())
                .input_message_content(InputMessageContent::InputMessageText(
                    InputMessageText::builder()
                        .text(FormattedText::builder().text(&text_to_send).build())
                        .build(),
                ))
                .build(),
        )
        .await
        .unwrap();

    let mut wait_messages: i32 = 100;
    while let Some(message) = receiver.recv().await {
        if let Update::NewMessage(new_message) = message.borrow() {
            log::info!("new message received: {:?}", new_message);
            if let MessageContent::MessageText(text) = new_message.message().content() {
                if text.text().text().eq(&text_to_send) {
                    break;
                }
            }
        }
        wait_messages -= 1;
        if wait_messages == 0 {
            break;
        }
    }

    client1.stop().await.unwrap();
    client2.stop().await.unwrap();

    loop {
        if worker.wait_client_state(&client1).await.unwrap() == ClientState::Closed {
            log::info!("client1 closed");
            break;
        }
    }

    loop {
        if worker.wait_client_state(&client2).await.unwrap() == ClientState::Closed {
            log::info!("client2 closed");
            break;
        }
    }

    worker.stop();
    log::info!("worker stopped");
}
