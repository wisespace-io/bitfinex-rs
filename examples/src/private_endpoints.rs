extern crate bitfinex;

use bitfinex::api::*;
use bitfinex::pairs::*;
use bitfinex::currency::*;

fn main() {
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());
    let api = Bitfinex::new(api_key, secret_key);
  
    // ORDERS
    match api.orders.active_orders() {
        Ok(orders) => {
            for order in &orders {
                println!("Active orders => Symbol: {:?} amount: {:?} price: {:?}", order.symbol, order.amount, order.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }     

    let order_history = api.orders.history(BTCUSD.to_owned()); // Use None if you don't want a pair 
    match order_history {
        Ok(orders) => {
            for order in &orders {
                println!("Order History => Symbol: {:?} amount: {:?} price: {:?}", order.symbol, order.amount, order.price);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }

    // WALLET
    match api.account.get_wallets() {
        Ok(wallets) => {
            for wallet in &wallets {
                println!("Wallet => Currency: {:?} Balance: {:?}", wallet.currency, wallet.balance);
            }    
        },
        Err(e) => println!("Error: {}", e),
    }

    // MARGIN INFO
    match api.account.margin_base() {
        Ok(info) => {
            println!("Margin Base Info => Profile/Loss: {:?}", info.margin.user_profit_loss);    
        },
        Err(e) => println!("Error: {}", e),
    }

    match api.account.margin_symbol(ETHUSD) {
        Ok(info) => {
            println!("Margin Symbol Info => Gross Balance: {:?}", info.margin.gross_balance);    
        },
        Err(e) => println!("Error: {}", e),
    }   

    // FUNDING INFO
    match api.account.funding_info(USD) {
        Ok(info) => {
            println!("Funding Info => Yield Loan: {:?} Yield Lend: {:?}", info.funding.yield_loan, info.funding.yield_lend);    
        },
        Err(e) => println!("Error: {}", e),
    }    
}