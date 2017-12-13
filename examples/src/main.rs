extern crate bitfinex;

use bitfinex::api::*;
use bitfinex::pairs::*;

fn main() {
    public_endpoints();
}

fn public_endpoints() {
    let api = Bitfinex::new(None, None);

    let trading_pair = api.ticker.trading_pair(ETHUSD.to_owned());
    match trading_pair {
        Ok(answer) => println!("bid: {:?}  ask: {:?}", answer.bid, answer.ask),
        Err(e) => println!("Error: {}", e),
    }   
}
