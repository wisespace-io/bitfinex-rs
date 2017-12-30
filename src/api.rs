use book::*;
use ticker::*;
use trades::*;
use candles::*;
use orders::*;

#[derive(Clone)]
pub struct Bitfinex {
    pub book: Book,
    pub ticker: Ticker,
    pub trades: Trades,
    pub candles: Candles,
    pub orders: Orders
}

impl Bitfinex {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Bitfinex { 
            book: Book::new(),
            ticker: Ticker::new(),
            trades: Trades::new(),
            candles: Candles::new(),
            orders: Orders::new(api_key, secret_key)
        }
    }
}