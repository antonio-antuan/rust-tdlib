# rust-tdlib
Rust client for TDlib.
Library allows you to interact with Telegram Database library.
Currently supports tdlib v1.7.0.

### Features
* client - provides total integration with TDlib API. See `examples` directory. Enabled by default. rust-tdlib provides only TDlib types without that feature. 

### Run example
1. [Build tdlib](https://core.telegram.org/tdlib/docs/#building).
2. [Register](https://my.telegram.org) your app and get `api_hash` and `api_id`.
3. `RUST_LOG=info API_ID=api_id API_HASH=api_hash cargo run --example main`
4. `RUST_LOG=info API_ID=api_id API_HASH=api_hash cargo run --example read_updates`
