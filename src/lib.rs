extern crate hyper;

extern crate serde_json;

use hyper::*;
use std::io::Read;

pub fn call(payload: &str) -> String {
    let client = Client::new();

    let mut res = client.post("http://node.steem.ws/rpc")
        .body(payload)
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
        let rpc_call = r#"{"jsonrpc": "2.0", "params": ["database_api", "get_dynamic_global_properties", []], "id":1, "method":"call"}"#;
        assert_eq!("", call(&rpc_call));
    }
}
