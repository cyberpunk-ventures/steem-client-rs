extern crate hyper;
extern crate serde_json;
extern crate env_logger;

use hyper::Client;
use serde_json::Map;
use std::io::Read;
use serde_json::builder::{ArrayBuilder, ObjectBuilder};

#[derive(Debug)]
pub enum SteemError {
    CallFailed,
    Http(hyper::Error),
    JsonParsing(serde_json::Error),
    ResponseIo(std::io::Error),
}

pub enum SteemApi {
    DatabaseApi,
    FollowsApi,
}

pub fn call(api: String,
            api_method: String,
            args: Vec<String>)
            -> Result<Map<String, serde_json::value::Value>, SteemdError> {
    const RPC_ENDPOINT: &'static str = "http://node.steem.ws/rpc";

    let params = ArrayBuilder::new()
        .push(api)
        .push(api_method)
        .push(args)
        .build();

    let value = ObjectBuilder::new()
        .insert("jsonrpc".to_string(), "2.0")
        .insert("method".to_string(), "call")
        .insert("id".to_string(), "1")
        .insert("params".to_string(), params)
        .build();

    let client = Client::new();

    let mut res = client.post(RPC_ENDPOINT)
        .body(&serde_json::to_string(&value).unwrap())
        .send()
        .unwrap();

    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    let json: Map<String, serde_json::value::Value> = serde_json::from_str(&s).unwrap();

    match json.contains_key("error") {
        false => Ok(json),
        true => Err(SteemdError::CallFailed),
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::*;

    #[cfg(test)]
    mod tests {
        extern crate serde_json;
        use super::*;

        #[test]
        fn get_dynamic_props_rpc_call_succeeds() {
            let api = SteemApi::DatabaseApi;
            let api_method = "get_dynamic_global_properties".to_string();
            let args = vec![];
            let response_map = call(api, api_method, args).unwrap();
            assert!(response_map["result"]["head_block_number"].as_u64().unwrap() > 10000000);
        }

        #[test]
        fn get_content_rpc_call_succeeds() {
            let api = SteemApi::DatabaseApi;
            let api_method = "get_content".to_string();
            let args = vec!["ontofractal".to_string(), "ann-introducing-glasnost-alpha-open-source-blog-and-app-server-for-steem-golos-blockchains".to_string()];
            let response_map = call(api, api_method, args).unwrap();
            assert!(response_map["result"]["title"].as_str().unwrap() == "[ANN] Introducing Glasnost alpha: open source blog and app server for Steem/Golos blockchains");
        }

        #[test]
        fn get_followers_rpc_call_succeeds() {
            let api = SteemApi::FollowsApi;
            let api_method = "get_followers".to_string();
            let args =
                vec!["ontofractal".to_string(), "".to_string(), "blog".to_string(), "100".to_string() ];
            let response_map = call(api, api_method, args).unwrap();
            assert!( !response_map["result"][0]["follower"].as_str().unwrap().is_empty() );
        }
    }
