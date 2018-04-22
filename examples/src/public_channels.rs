extern crate bitfinex;

use bitfinex::websockets::*;

fn main() {
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.connect().unwrap(); // check error

}