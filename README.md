# Steem-client-rs

This is a Steem client library for Rust. Work in progress.

# How to use

Steem-client-rs hasn't been graduated to crates.io yet, so you'll need to use git dependency, like this.

```rust
[dependencies]
steem-client = { git = "https://github.com/cyberpunk-ventures/steem-client-rs" }
```

Example code
```rust
extern crate steem_client;
use steem_client::*;

let api = steem_client::SteemApi::DatabaseApi;
let api_method = "get_dynamic_global_properties".to_string();
let args = vec![];

let response: Result<serde_json::Value, SteemError> = steem_client::call(api, api_method, args);
response["result"]["head_block_number"].as_u64().unwrap() > 10000000; // true
```

# Roadmap

* Implement futures
* Investigate json_rpc crate from eth_core implementation in Ethereum's Parity
* Add more utility functions
* Investigate WS and evented approach
* Add more response structs and types for different JSONRPC apis and methods
* More tests and docs
