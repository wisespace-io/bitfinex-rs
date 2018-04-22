use errors::*;
use url::Url;

use tungstenite::connect;
use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;

static WEBSOCKET_URL: &'static str = "wss://api.bitfinex.com/ws/2";

#[derive(Default)]
pub struct WebSockets {
    socket: Option<(WebSocket<AutoStream>, Response)>,
}

impl WebSockets {
    pub fn new() -> WebSockets {
        WebSockets {
            socket: None,
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        let wss: String = format!("{}", WEBSOCKET_URL);
        let url = Url::parse(&wss)?;

        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => {
                bail!(format!("Error during handshake {}", e));
            }
        }
    }
}