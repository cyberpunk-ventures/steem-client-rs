extern crate hyper;
extern crate rustc_serialize;
#[macro_use]
extern crate json;

use hyper::*;
use std::io::Read;
// use rustc_serialize::json::Json;


pub fn call(api: String, api_method: String, args: Vec<String>) -> String {
    let client = Client::new();

    let data = object!{
        "jsonrpc" => "2.0",
        "method" => "call",
        "id" => 1,
        "params" => array![api, api_method, args]
    };

    let json = data.dump();
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
        let api = "database_api".to_string();
        let api_method = "get_dynamic_global_properties".to_string();
        let args = vec![];
        let result_str = call(api, api_method, args);
        println!("{:?}", result_str);
        assert!(result_str.contains("head_block_number"));
    }
}
