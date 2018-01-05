use book::*;
use ticker::*;
use trades::*;
use candles::*;
use orders::*;
use account::*;

#[derive(Clone)]
pub struct Bitfinex {
    pub book: Book,
    pub ticker: Ticker,
    pub trades: Trades,
    pub candles: Candles,
    pub orders: Orders,
    pub account: Account
}

impl Bitfinex {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Bitfinex { 
            book: Book::new(),
            ticker: Ticker::new(),
            trades: Trades::new(),
            candles: Candles::new(),
            orders: Orders::new(api_key.clone(), secret_key.clone()),
            account: Account::new(api_key.clone(), secret_key.clone())
        }
    }
}