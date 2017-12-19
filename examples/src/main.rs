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

    let funding_currency = api.ticker.funding_currency("USD".to_owned());
    match funding_currency {
        Ok(answer) => println!("bid: {:?}  ask: {:?}", answer.bid, answer.ask),
        Err(e) => println!("Error: {}", e),
    }

    let trading_pairs = api.trades.trading_pair(ETHUSD.to_owned());
    match trading_pairs {
        Ok(trades) => {
            for trade in &trades {
                println!("Trading => amount: {:?}  price: {:?}", trade.amount, trade.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }   

    let funding_currency = api.trades.funding_currency("USD".to_owned());
    match funding_currency {
        Ok(trades) => {
            for trade in &trades {
                println!("Funding => amount: {:?}  price: {:?}", trade.amount, trade.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }             
}
