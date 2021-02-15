# rust-tdlib
Rust client for TDlib.
Library allows you to interact with Telegram Database library.
Currently supports tdlib v1.7.0.

### Features
* client - provides total integration with TDlib API. See `examples` directory. Enabled by default. rust-tdlib provides only TDlib types without that feature. 

### Run example
1. Build tdlib: https://core.telegram.org/tdlib/docs/#building. See the docs for rtdlib-sys for more details.
2. Register your app: https://my.telegram.org and get `api_hash` and `api_id`.
3. `RUST_LOG=info API_ID=api_hash API_HASH=api_id cargo run client --example main`
4. `RUST_LOG=info API_ID=api_hash API_HASH=api_id cargo run client --example read_updates`
