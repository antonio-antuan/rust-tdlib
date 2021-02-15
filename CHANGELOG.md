CHANGELOG
===
## 0.2.0
* Support for tdlib 1.7.0
* API changed because of need for support multiple clients: you can create multiple clients and bind it with one worker. See examples/main.rs for details.
* Updates received with tokio::mpsc::channel are exactly types::Update instead of types::TdType. 

## 0.1.2
* Fixed with_auth_state_handler for ClientBuilder.
* Documentation improvements.

## 0.1.1
* Improvements for docs.rs
* `client` feature enabled by default
* possibility to write tdlib logs added

## 0.1.0
Initial library version. 