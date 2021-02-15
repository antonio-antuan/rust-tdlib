use log;
use rust_tdlib::{
    client::{Client, Worker},
    tdjson,
    types::{GetMe, SearchPublicChats, TdlibParameters},
};

#[tokio::main]
async fn main() {
    tdjson::set_log_verbosity_level(1);
    env_logger::init();
    let client1 = Client::builder()
        .with_tdlib_parameters(
            TdlibParameters::builder()
                .database_directory("tdlib1")
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

    let client2 = Client::builder()
        .with_tdlib_parameters(
            TdlibParameters::builder()
                .database_directory("tdlib2")
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

    let me = client1.get_me(GetMe::builder().build()).await.unwrap();
    log::info!("me 1: {:?}", me);

    let (client2_state, client2) = worker.auth_client(client2).await.unwrap();
    log::info!("client2 authorized");

    let me = client2.get_me(GetMe::builder().build()).await.unwrap();
    log::info!("me 2: {:?}", me);

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

    client1.stop().await.unwrap();
    client2.stop().await.unwrap();
    client1_state.await.unwrap();
    log::info!("client1 closed");

    client2_state.await.unwrap();
    log::info!("client1 closed");

    worker.stop();
    log::info!("worker stopped");

    waiter.await.unwrap();
    log::info!("waiter stopped");
}
