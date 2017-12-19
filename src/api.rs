use ticker::*;
use trades::*;
use errors::*;

#[derive(Clone)]
pub struct Bitfinex {
    pub ticker: Ticker,
    pub trades: Trades
}

impl Bitfinex {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Bitfinex { 
            ticker: Ticker::new(),
            trades: Trades::new()
        }
    }
}