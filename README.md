# rust-tdlib
Rust client for TDlib.
Library allows you to interact with Telegram Database library. 

### Features
* default - provides only TDlib types
* client - provides total integration with TDlib API. See `examples` directory.

### Run example
1. Register your app: https://my.telegram.org and get `api_hash` and `api_id`.
2. `RUST_LOG=info API_ID=api_hash API_HASH=api_id TG_PHONE=your_phone_number cargo run --features client --example main`