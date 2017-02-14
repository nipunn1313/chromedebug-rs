extern crate websocket;

use websocket::{
    Message,
    Receiver,
    Sender,
    Server as WSServer,
};

pub struct Server {}

impl Server {
    pub fn run() {
        let server = WSServer::bind("localhost:9222").unwrap();

        for connection in server {
            let request = connection.unwrap().read_request().unwrap();
            let headers = request.headers.clone();

            println!("Request: {:?}, Headers: {:?}", request.key(), headers);

            request.validate().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
