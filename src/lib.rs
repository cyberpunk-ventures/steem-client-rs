extern crate hyper;
extern crate hyper_openssl;

#[macro_use]
extern crate serde_json;
extern crate env_logger;

use std::io::Read;

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

pub fn call(api: SteemApi,
            api_method: String,
            args: Vec<String>)
            -> Result<serde_json::Value, SteemError> {
    const RPC_ENDPOINT: &'static str = "https://steemd.steemit.com/rpc";

    let api_str: String = match api {
        SteemApi::DatabaseApi => "database_api".to_string(),
        SteemApi::FollowsApi => "follow_api".to_string(),
    };

    let value = json!({
        "jsonrpc": "2.0",
        "method": "call",
        "id": "1",
        "params": [api_str, api_method, args]
    });

    let ssl = hyper_openssl::OpensslClient::new().unwrap();
    let connector = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(connector);

    let mut res = try!(client.post(RPC_ENDPOINT)
        .body(&serde_json::to_string(&value).unwrap())
        .send()
        .map_err(SteemError::Http));

    let mut s = String::new();
    try!(res.read_to_string(&mut s).map_err(SteemError::ResponseIo));
    let json: serde_json::Value = try!(serde_json::from_str(&s).map_err(SteemError::JsonParsing));

    match json["error"].is_string() {
        false => Ok(json),
        true => Err(SteemError::CallFailed),
    }
}


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
