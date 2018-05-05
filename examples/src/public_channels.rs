extern crate bitfinex;

use bitfinex::events::*;
use bitfinex::websockets::*;
use bitfinex::pairs::*;
use bitfinex::currency::*;

struct WebSocketHandler;

impl EventHandler for WebSocketHandler {
    fn on_connect(&mut self, event: NotificationEvent) {
        println!("{:?}", event);
    }

    fn on_subscribed(&mut self, event: NotificationEvent) {
        println!("{:?}", event);
    }

    fn on_data_event(&mut self, event: DataEvent) {
        println!("{:?}", event);
    }
}

fn main() {
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_event_handler(WebSocketHandler);
    web_socket.connect().unwrap(); // check error

    web_socket.subscribe_ticker(BTCUSD, EventType::Trading);
    web_socket.subscribe_ticker(USD, EventType::Funding);

    web_socket.event_loop().unwrap(); // check error
}