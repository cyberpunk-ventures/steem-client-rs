# Steem-client-rs

This is a Steem client library written in Rust language.

Work in progress and under development.

# How to use

Steem-client-rs hasn't been graduated to crates.io yet, so you'll need to use git dependency, like this.

```rust
[dependencies]
steem_client = { git = "https://github.com/cyberpunk-ventures/steem-client-rs" }
```

Example code
```rust
extern crate steem_client;
use steem::*;

let api = "database_api".to_string();
let api_method = "get_dynamic_global_properties".to_string();
let args = vec![];

let result_map: Map<String, serde_json::value::Value> = call(api, api_method, args)
        .unwrap();
```
