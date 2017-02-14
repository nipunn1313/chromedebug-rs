extern crate websocket;

use websocket::{
    Message,
    Receiver,
    Sender,
    Server as WSServer,
};

use websocket::message::Type;
use websocket::header::WebSocketProtocol;

extern crate rustc_serialize;
use rustc_serialize::json;

use std::collections::BTreeMap;
use std::str;
use std::borrow::Borrow;

pub struct Server {}

#[derive(Debug, RustcDecodable)]
struct MethodCall {
    id: u32,
    method: String,
}

#[derive(Debug, RustcEncodable)]
struct MethodResult {
    id: u32,
    result: BTreeMap<String, json::Json>,
}

impl Server {
    pub fn run() {
        let server = WSServer::bind("localhost:9223").unwrap();

        for connection in server {
            let request = connection.unwrap().read_request().unwrap();
            let headers = request.headers.clone();

            println!("Request: {:?}, Headers: {:?}", request.key(), headers);

            request.validate().unwrap();

            let mut response = request.accept(); // Form a response
            if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
				if protocols.contains(&("rust-websocket".to_string())) {
					// We have a protocol we want to use
					response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
				}
			}

            let mut client = response.send().unwrap(); // Send the response

            let ip = client.get_mut_sender()
				.get_mut()
				.peer_addr()
				.unwrap();

			println!("Connection from {}", ip);

			// let message: Message = Message::text("Hello".to_string());
			// client.send_message(&message).unwrap();

			let (mut sender, mut receiver) = client.split();

			for message in receiver.incoming_messages() {
				let message: Message = message.unwrap();

                // let outgoing_message = Message::text("{}".to_string());
                // sender.send_message(&outgoing_message).unwrap();

				match message.opcode {
					Type::Close => {
						let message = Message::close();
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					},
					Type::Ping => {
						let message = Message::pong(message.payload);
						sender.send_message(&message).unwrap();
					}
                    Type::Text => {
                        println!("GOT TEXT {:?}", message);
                        let utf8 = str::from_utf8(message.payload.borrow()).unwrap();
                        let json: MethodCall = json::decode(&utf8).unwrap();
                        println!("json: {:?}", json);

                        let response = MethodResult { id: json.id, result: BTreeMap::new() };
                        println!("response: {:?}", response);
                        sender.send_message(&Message::text(json::encode(&response).unwrap())).unwrap();

                    }
					_ => unimplemented!()
				}
			}
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
