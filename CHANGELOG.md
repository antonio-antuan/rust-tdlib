CHANGELOG
===
## 0.3.0
* New methods to handle and observe client state changes: `Worker::get_client_state`, `Worker::reset_auth`, `Worker::wait_auth_state_change`, `Worker::wait_client_state`. See `examples/handle_auth_state.rs` for details.
* `JoinHandle` returned by `Worker::start` now yields empty tuple.  
* `Worker::auth_client` renamed to `Worker::bind_client` and does not return `JoinHandle`.
* `SignalAuthStateHandler` added.
* `ClientState` now does not contain `Error` variant, `Authorizing` variant added.
* `RTDError::TdlibError(String)` changed to `RTDError::TDLibError(rust_tdlib::types::error::Error)`.

## 0.2.3
* Fixed enums serializing

## 0.2.1
* Readme fixes
* Fixed stack overflow error

## 0.2.0
* Support for tdlib 1.7.0
* API changed because of need for support multiple clients: you can create multiple clients and bind it with one worker. See examples/main.rs for details.
* Updates received with tokio::mpsc::channel are exactly types::Update instead of types::TdType.
* Minor fixes and improvements.

## 0.1.2
* Fixed with_auth_state_handler for ClientBuilder.
* Documentation improvements.

## 0.1.1
* Improvements for docs.rs
* `client` feature enabled by default
* possibility to write tdlib logs added

## 0.1.0
Initial library version. 