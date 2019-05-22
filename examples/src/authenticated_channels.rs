extern crate bitfinex;

use bitfinex::{ errors::*, events::*, websockets::* };

struct WebSocketHandler;

impl EventHandler for WebSocketHandler {
    fn on_connect(&mut self, event: NotificationEvent) {
        if let NotificationEvent::Info(info) = event {
            println!("Platform status: {:?}, Version {}", info.platform, info.version);
        }
    }

    fn on_auth(&mut self, event: NotificationEvent) {
        if let NotificationEvent::Auth(auth) = event {
            println!("Auth {}: {:?}", auth.status, auth.msg);
        }
    }

    fn on_subscribed(&mut self, _event: NotificationEvent) {}

    fn on_data_event(&mut self, _event: DataEvent) {}

    fn on_error(&mut self, message: Error) {
        println!("{:?}", message);
    }
}

fn main() {
    let api_key = "YOUR_API_KEY";
    let secret_key = "YOUR_SECRET_KEY";
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_event_handler(WebSocketHandler);
    web_socket.connect().unwrap(); // check error

    web_socket.auth(api_key, secret_key, false).unwrap();

    // TODO: Handle authenticated channels

    web_socket.event_loop().unwrap(); // check error
}
