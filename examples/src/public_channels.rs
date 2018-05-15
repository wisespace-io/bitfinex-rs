extern crate bitfinex;

use bitfinex::{ errors::*, events::*, websockets::* };
use bitfinex::{ pairs::*, currency::*, precision::* };

struct WebSocketHandler;

impl EventHandler for WebSocketHandler {
    fn on_connect(&mut self, event: NotificationEvent) {
        if let NotificationEvent::Info{event:_, version, platform } = event {
            println!("Platform status: {:?}, Version {}", platform.status, version);
        }
    }

    fn on_subscribed(&mut self, event: NotificationEvent) {
        if let NotificationEvent::TradingSubsbribed{ event:_, channel, chan_id, .. } = event {
            println!("Subscribed to Trading Channel: {:?}, Channel ID: {}", channel, chan_id);
        } else if let NotificationEvent::FundingSubsbribed{ event:_, channel, chan_id, .. } = event {
            println!("Subscribed to Funding Channel: {:?}, Channel ID: {}", channel, chan_id);
        } else if let NotificationEvent::CandlesSubsbribed{ event:_, channel, chan_id, key } = event {
            println!("Subscribed to Channel: {:?}, Channel ID: {}, Key: {}", channel, chan_id, key);
        }
    }

    fn on_data_event(&mut self, event: DataEvent) {
        if let DataEvent::TickerTradingEvent(channel, trading) = event {
            println!("Ticker Trading ({})- Bid {:?}, Ask: {}", channel, trading.bid, trading.ask);
        } else if let DataEvent::TickerFundingEvent(channel, trading) = event {
            println!("Ticker Funding ({}) - Bid {:?}, Ask: {}", channel, trading.bid, trading.ask);
        }
        // ... Add for all events you have subscribed (Trades, Books, ...)
    }

    fn on_error(&mut self, message: Error) {
        println!("{:?}", message);
    }    
}

fn main() {
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_event_handler(WebSocketHandler);
    web_socket.connect().unwrap(); // check error

    // TICKER
    web_socket.subscribe_ticker(BTCUSD, EventType::Trading);
    web_socket.subscribe_ticker(USD, EventType::Funding);

    // TRADES
    web_socket.subscribe_trades(BTCUSD, EventType::Trading);
    web_socket.subscribe_trades(USD, EventType::Funding);

    // BOOKS
    web_socket.subscribe_books(BTCUSD, EventType::Trading, P0, "F0", 25);
    web_socket.subscribe_books(USD, EventType::Funding, P0, "F0", 25);

    // CANDLES
    web_socket.subscribe_candles(BTCUSD, "1m");

    web_socket.event_loop().unwrap(); // check error
}