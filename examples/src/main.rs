extern crate bitfinex;

use bitfinex::api::*;
use bitfinex::pairs::*;
use bitfinex::currency::*;
use bitfinex::precision::*;

fn main() {
    public_endpoints();
    
    println!("=>> Private Endpoints: ");

    private_endpoints();
}

fn public_endpoints() {
    let api = Bitfinex::new(None, None);
  
    // TICKER
    let trading_pair = api.ticker.trading_pair(ETHUSD);
    match trading_pair {
        Ok(answer) => println!("bid: {:?}  ask: {:?}", answer.bid, answer.ask),
        Err(e) => println!("Error: {}", e),
    }   

    let funding_currency = api.ticker.funding_currency(USD);
    match funding_currency {
        Ok(answer) => println!("bid: {:?}  ask: {:?}", answer.bid, answer.ask),
        Err(e) => println!("Error: {}", e),
    }

    // TRADES
    let trading_pairs = api.trades.trading_pair(ETHUSD);
    match trading_pairs {
        Ok(trades) => {
            for trade in &trades {
                println!("Trading => amount: {:?}  price: {:?}", trade.amount, trade.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }   

    let funding_currency = api.trades.funding_currency(USD);
    match funding_currency {
        Ok(trades) => {
            for trade in &trades {
                println!("Funding => amount: {:?}  price: {:?}", trade.amount, trade.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }        

    // BOOK
    let trading_pairs = api.book.trading_pair(ETHUSD, P0);
    match trading_pairs {
        Ok(books) => {
            for book in &books {
                println!("Trading => price: {:?} amount: {:?}", book.price, book.amount);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }   

    let funding_currency = api.book.funding_currency(USD, P0);
    match funding_currency {
        Ok(books) => {
            for book in &books {
                println!("Funding => rate: {:?} amount: {:?}", book.rate, book.amount);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }  

    // CANDLES
    let last = api.candles.last(ETHUSD, "1m");
    match last {
        Ok(answer) => println!("Candle Last => High: {:?} low: {:?}", answer.high, answer.low),
        Err(e) => println!("Error: {}", e),
    }    

    let history = api.candles.history(ETHUSD, "12h");
    match history {
        Ok(candles) => {
            for candle in &candles {
                println!("Candle History => High: {:?} Low: {:?}", candle.high, candle.low);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }             
}

fn private_endpoints() {
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());
    let api = Bitfinex::new(api_key, secret_key);
  
    // ORDERS
    let active_orders = api.orders.active_orders();
    match active_orders {
        Ok(orders) => {
            for order in &orders {
                println!("Active orders => Symbol: {:?} amount: {:?} price: {:?}", order.symbol, order.amount, order.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }     

    let order_history = api.orders.history(BTCUSD.to_owned()); // Use None if you don't want a par 
    match order_history {
        Ok(orders) => {
            for order in &orders {
                println!("Order History => Symbol: {:?} amount: {:?} price: {:?}", order.symbol, order.amount, order.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }
}