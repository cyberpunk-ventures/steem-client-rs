extern crate hyper;
extern crate serde_json;
extern crate env_logger;

use hyper::*;
use serde_json::Map;
use std::io::Read;
use serde_json::builder::{ArrayBuilder, ObjectBuilder};

pub fn call(api: String,
            api_method: String,
            args: Vec<String>)
            -> Map<String, serde_json::value::Value> {

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

    let mut res = client.post("http://node.steem.ws/rpc")
        .body(&serde_json::to_string(&value).unwrap())
        .send()
        .unwrap();

    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    let json: Map<String, serde_json::value::Value> = serde_json::from_str(&s).unwrap();

    json
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_json_rpc_call_works() {

        let api = "database_api".to_string();
        let api_method = "get_dynamic_global_properties".to_string();
        let args = vec![];
        let result_map = call(api, api_method, args);

        assert!(result_map.contains_key("id"));
    }
}
