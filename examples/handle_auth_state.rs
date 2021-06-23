use rust_tdlib::types::AuthorizationState;
use rust_tdlib::{
    client::{Client, ClientState, SignalAuthStateHandler, Worker},
    tdjson,
    types::TdlibParameters,
};

#[tokio::main]
async fn main() {
    tdjson::set_log_verbosity_level(1);
    env_logger::init();

    let db_dir = "tddb_auth";

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
        .with_auth_state_channel(10)
        .build()
        .unwrap();

    let (sx, rx) = tokio::sync::mpsc::channel(5);
    let auth_handler = SignalAuthStateHandler::new(rx);
    let mut worker = Worker::builder()
        .with_auth_state_handler(auth_handler)
        .build()
        .unwrap();

    worker.start();

    // Two sends below are common for TDLib authorization flow.
    // You can check it calling `Worker::wait_auth_state_change` (see below) unless ClientState::Opened received.
    sx.send("".to_string()).await.unwrap(); // empty encryption key
    sx.send("invalid phone".to_string()).await.unwrap(); // wrong phone number: not a number

    let client = worker.bind_client(client).await.expect("can't bind client");

    loop {
        match worker.wait_auth_state_change(&client).await {
            Ok(res) => {
                match res {
                    Ok(state) => match state {
                        ClientState::Opened => {
                            log::info!("client authorized; can start interaction");
                        }
                        ClientState::Closed => {
                            log::warn!("client closed, need to reauthorize it");
                            break;
                        }
                        ClientState::Authorizing => {
                            log::info!("client not authorized yet")
                        }
                    },
                    Err((err, auth_state)) => {
                        match &auth_state.authorization_state() {
                            AuthorizationState::WaitPhoneNumber(_) => {
                                log::info!("received error when setup phone number: {}", err);
                                // send correct phone number
                                sx.send(env!("PHONE_NUMBER").to_string()).await.unwrap();
                                // and handle auth state manually again
                                worker
                                    .handle_auth_state(&auth_state.authorization_state(), &client)
                                    .await
                                    .expect("can't handle it");
                                // on this step tdlib usually waits for auth code, which sent by special telegram account
                                sx.send("incorrect empty code".to_string()).await.unwrap();
                            }
                            AuthorizationState::WaitCode(_) => {
                                log::error!("received code error, need to handle it here");
                                break;
                            }
                            _ => {
                                log::error!(
                                    "state: {:?}, error: {:?}",
                                    auth_state.authorization_state(),
                                    err
                                );
                                break;
                            }
                        }
                    }
                }
            }
            Err(err) => {
                log::error!("cannot wait for auth state changes: {}", err);
                break;
            }
        }
    }

    let (client_state, auth_state) = worker.get_client_state(&client).await.unwrap();
    assert_eq!(client_state, ClientState::Authorizing);

    match &auth_state {
        AuthorizationState::WaitCode(_) => {
            // `WaitCode`, as expected
        }
        _ => {
            panic!("weird, we've just seen that auth flow stopped on `WaitCode` state")
        }
    }

    // and well, we can just wait for client state changes, without any details about authorization state.
    let res = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        match worker.wait_client_state(&client).await.unwrap() {
            ClientState::Opened => {
                log::info!("here we go")
            }
            ClientState::Closed => {
                log::error!("client unexpectedly closed")
            }
            ClientState::Authorizing => {
                log::warn!("client is not authorized yet")
            }
        };
    })
    .await;
    // of course here we await infinitely because of not handled WaitCode above.
    assert!(res.is_err());
}
