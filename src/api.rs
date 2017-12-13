use ticker::*;
use errors::*;

#[derive(Clone)]
pub struct Bitfinex {
    pub ticker: Ticker
}

impl Bitfinex {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Bitfinex { 
            ticker: Ticker::new() 
        }
    }
}