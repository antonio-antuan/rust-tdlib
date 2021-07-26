use rust_tdlib::client::{Client, Worker, ClientState};
use rust_tdlib::types::*;
use rust_tdlib::client::SignalAuthStateHandler;
use rust_tdlib::client::tdlib_client::TdJson;
use rust_tdlib::tdjson;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let (client, worker) = setup().await;
    client.get_me(GetMe::builder().build()).await.expect("can't GetMe");

}

async fn setup() -> (Client<TdJson>, Worker<SignalAuthStateHandler, TdJson>) {
    tdjson::set_log_verbosity_level(1);
    let db_dir = "tddb_test";

    if std::path::Path::new(db_dir).exists() {
        // We need to check auth errors and so on so we need to ensure that authorization not done yet.
        std::fs::remove_dir_all(db_dir).expect("can't clear directory");
    }

    let tdlib_parameters = TdlibParameters::builder()
        .database_directory(db_dir)
        .use_test_dc(true)
        .api_id(env!("API_ID").parse::<i32>().unwrap())
        .api_hash(env!("API_HASH"))
        .system_language_code("en")
        .device_model("Desktop")
        .system_version("Unknown")
        .application_version(env!("CARGO_PKG_VERSION"))
        .enable_storage_optimizer(true)
        .build();

    let client = Client::builder()
        .with_tdlib_parameters(tdlib_parameters)
        .with_auth_state_channel(5)
        .build()
        .unwrap();

    let (sx, rx) = tokio::sync::mpsc::channel(5);
    let auth_handler = SignalAuthStateHandler::new(rx);
    let mut worker = Worker::builder()
        .with_auth_state_handler(auth_handler)
        .build()
        .unwrap();
    println!("worker start");
    worker.start();
    println!("worker started");

    sx.send("".to_string()).await.unwrap();
    sx.send("9996612222".to_string()).await.unwrap();

    println!("bind client");
    let client = worker.bind_client(client).await.expect("can't bind client");
    log::debug!("client bound");

    loop {
        match worker.wait_auth_state_change(&client).await {
            Ok(res) => {
                match res {
                    Ok(state) => match state {
                        ClientState::Opened => {
                            log::debug!("client opened");
                            break;
                        }
                        ClientState::Closed => {
                            panic!("client closed, need to reauthorize it");
                        }
                        ClientState::Authorizing => {
                            log::debug!("client not authorized yet")
                        }
                    },
                    Err((err, auth_state)) => {
                        panic!("cannot authorize, error={:?}, state={:?}", err, auth_state)
                    }
                }
            }
            Err(err) => {
                panic!("cannot wait for auth state changes: {}", err);
            }
        }
    }
    (client, worker)
}