extern crate hyper;
extern crate rustc_serialize;
#[macro_use]
extern crate json;

use hyper::*;
use std::io::Read;
// use rustc_serialize::json::Json;


pub fn call(api: String, api_method: String, args: Vec<String>) -> json::JsonValue {
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
    let obj = json::parse(&s).unwrap();
    obj
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
        let head_block_number = result_str["result"]["head_block_number"].as_fixed_point_u64(0);
        println!("{:?}", head_block_number);
        assert!(head_block_number.unwrap() > 3990000);
    }
}
