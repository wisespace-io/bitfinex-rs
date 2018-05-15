use url::Url;
use errors::*;
use events::*;
use serde_json::{from_str};

use tungstenite::connect;
use tungstenite::Message;
use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;

use std::sync::mpsc::{self, channel};

static INFO: &'static str = "info";
static SUBSCRIBED: &'static str = "subscribed";
static WEBSOCKET_URL: &'static str = "wss://api.bitfinex.com/ws/2";

pub trait EventHandler {
    fn on_connect(&mut self, event: NotificationEvent);
    fn on_subscribed(&mut self, event: NotificationEvent);
    fn on_data_event(&mut self, event: DataEvent);
    fn on_error(&mut self, message: Error); 
}

pub enum EventType {
    Funding,
    Trading
}

#[derive(Debug)]
enum WsMessage {
    Close,
    Text(String),
}

pub struct WebSockets {
    socket: Option<(WebSocket<AutoStream>, Response)>,
    sender: Sender,
    rx: mpsc::Receiver<WsMessage>,
    event_handler: Option<Box<EventHandler>>, 
}

impl WebSockets {
    pub fn new() -> WebSockets {
        let (tx, rx) = channel::<WsMessage>();
        let sender = Sender {
            tx: tx
        };

        WebSockets {
            socket: None,
            sender: sender,
            rx: rx,
            event_handler: None
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
                bail!(format!("Error during handshake {}", e))
            }
        }
    }

    pub fn add_event_handler<H>(&mut self, handler: H) where H: EventHandler + 'static {
        self.event_handler = Some(Box::new(handler));
    }

    pub fn subscribe_ticker<S>(&mut self, symbol: S, et: EventType) where S: Into<String> {
        let local_symbol = self.format_symbol(symbol.into(), et);
        let msg = json!({"event": "subscribe", "channel": "ticker", "symbol": local_symbol });

        if let Err(error_msg) = self.sender.send(&msg.to_string()) {
            self.error_hander(error_msg);
        }
    }

    pub fn subscribe_trades<S>(&mut self, symbol: S, et: EventType) where S: Into<String> {
        let local_symbol = self.format_symbol(symbol.into(), et);
        let msg = json!({"event": "subscribe", "channel": "trades", "symbol": local_symbol });

        if let Err(error_msg) = self.sender.send(&msg.to_string()) {
            self.error_hander(error_msg);
        }
    }

    pub fn subscribe_candles<S>(&mut self, symbol: S, timeframe: S) where S: Into<String> {
        let key: String = format!("trade:{}:t{}", timeframe.into(), symbol.into());
        let msg = json!({"event": "subscribe", "channel": "candles", "key": key });

        if let Err(error_msg) = self.sender.send(&msg.to_string()) {
            self.error_hander(error_msg);
        } 
    }

    pub fn subscribe_books<S, P, F>(&mut self, symbol: S, et: EventType, prec: P, freq: F, len: u32) 
        where S: Into<String>, P: Into<String>, F: Into<String> 
    {
        let msg = json!(
            {
                "event": "subscribe", 
                "channel": "book", 
                "symbol": self.format_symbol(symbol.into(), et),
                "prec": prec.into(),
                "freq": freq.into(),
                "len": len
            });

        if let Err(error_msg) = self.sender.send(&msg.to_string()) {
            self.error_hander(error_msg);
        }
    }

    fn error_hander(&mut self, error_msg: Error) {
        if let Some(ref mut h) = self.event_handler {
            h.on_error(error_msg);
        }        
    }

    fn format_symbol(&mut self, symbol: String, et: EventType) -> String {
        let local_symbol = match et {
            EventType::Funding => format!("f{}", symbol),
            EventType::Trading => format!("t{}", symbol),
        };

        local_symbol
    }

    pub fn event_loop(&mut self) -> Result<()>  {
        loop {
            if let Some(ref mut socket) = self.socket {
                loop {
                    match self.rx.try_recv() {
                        Ok(msg) => {
                            match msg {
                                WsMessage::Text(text) => {
                                    socket.0.write_message(Message::Text(text))?;
                                }
                                WsMessage::Close => {
                                    return socket.0.close(None).map_err(|e| e.into());
                                }
                            }
                        }
                        Err(mpsc::TryRecvError::Disconnected) => {
                            bail!("Disconnected")
                        }
                        Err(mpsc::TryRecvError::Empty) => break,
                    }
                }

                let message = socket.0.read_message()?;

                match message {
                    Message::Text(text) => {
                        if let Some(ref mut h) = self.event_handler {
                            if text.find(INFO) != None {
                                let event: NotificationEvent = from_str(&text)?;
                                h.on_connect(event);
                            } else if text.find(SUBSCRIBED) != None {
                                let event: NotificationEvent = from_str(&text)?;
                                h.on_subscribed(event);
                            } else {
                                let event: DataEvent = from_str(&text)?;
                                if let DataEvent::HeartbeatEvent(_a,_b) = event {
                                    continue;
                                } else {
                                    h.on_data_event(event);
                                }
                            }
                        }
                    }
                    Message::Binary(_) => {}
                    Message::Ping(_) |
                    Message::Pong(_) => {}
                }
            }
        }
    } 
}


#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<WsMessage>
}

impl Sender {
    pub fn send(&self, raw: &str) -> Result<()> {
        self.tx.send(WsMessage::Text(raw.to_string()))
            .map_err(|e| Error::with_chain(e, "Not able to send a message"))?;
        Ok(())
    }

    pub fn shutdown(&self) -> Result<()> {
        self.tx.send(WsMessage::Close)
            .map_err(|e| Error::with_chain(e, "Error during shutdown"))
    }
}
