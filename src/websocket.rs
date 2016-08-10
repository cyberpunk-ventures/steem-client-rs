// extern crate ws;
// extern crate env_logger;
// extern crate openssl;
//
// use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode, Error};
//
// // Our Handler struct.
// // Here we explicity indicate that the Client needs a Sender,
// // whereas a closure captures the Sender for us automatically.
// struct Client {
//     out: Sender,
// }
//
// // We implement the Handler trait for Client so that we can get more
// // fine-grained control of the connection.
// impl Handler for Client {
//     // `on_open` will be called only after the WebSocket handshake is successful
//     // so at this point we know that the connection is ready to send/receive messages.
//     // We ignore the `Handshake` for now, but you could also use this method to setup
//     // Handler state or reject the connection based on the details of the Request
//     // or Response, such as by checking cookies or Auth headers.
//     fn on_open(&mut self, _: Handshake) -> Result<()> {
//         println!("Entering on_open");
//         let request = r#"{"jsonrpc": "2.0", "params": ["database_api", "get_dynamic_global_properties", []], "id":1, "method":"call"}"#;
//         self.out.send(request)
//     }
//
//     // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
//     // and returns a `Result<()>`.
//     fn on_message(&mut self, msg: Message) -> Result<()> {
//         println!("Entering on_message");
//         // Close the connection when we get a response from the server
//         println!("Got message: {}", msg);
//         // self.out.close(CloseCode::Normal)
//
//         Ok(())
//     }
//
//
//     fn on_close(&mut self, code: CloseCode, reason: &str) {
//         match code {
//             CloseCode::Normal => println!("The client is done with the connection."),
//             CloseCode::Away => println!("The client is leaving the site."),
//             CloseCode::Abnormal => {
//                 println!("Closing handshake failed! Unable to obtain closing status from client.")
//             }
//             _ => println!("The client encountered an error: {}", reason),
//         }
//
//     }
//
//     fn on_error(&mut self, err: Error) {
//         println!("The server encountered an error: {:?}", err);
//     }
// }
//
// fn main() {
//     env_logger::init().unwrap();
//     // Now, instead of a closure, the Factory returns a new instance of our Handler.
//     println!("Entering main");
//     let ws_url = "wss://node.steem.ws";
//     if let Err(error) = connect(ws_url, |out| Client { out: out }) {
//         // Inform the user of failure
//         println!("Failed to create WebSocket due to: {:?}", error);
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn main() {}
// }
