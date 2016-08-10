extern crate hyper;
extern crate rustc_serialize;

use hyper::*;
use std::io::Read;
use rustc_serialize::{Decodable, Encodable, json};
use rustc_serialize::json::Json;

#[derive(RustcDecodable, RustcEncodable)]
struct JsonRpcStruct {
    jsonrpc: String,
    method: String,
    params: (String, String, Vec<String>),
    id: i64,
}

pub fn call(params: (String, String, Vec<String>)) -> String {
    let client = Client::new();

    let js_obj = JsonRpcStruct {
        jsonrpc: "2.0".to_string(),
        method: "call".to_string(),
        params: params,
        id: 1,
    };

    let json = json::encode(&js_obj).unwrap();
    let mut res = client.post("http://node.steem.ws/rpc")
        .body(&json)
        .send()
        .unwrap();

    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_json_rpc_call_works() {
        let plugin = "database_api".to_string();
        let method = "get_dynamic_global_properties".to_string();
        let rpc_call = (plugin, method, vec![]);
        let result_str = call(rpc_call);
        assert!(result_str.contains("head_block_number"));
    }
}
