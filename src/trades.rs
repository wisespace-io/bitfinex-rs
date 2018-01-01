use client::*;
use errors::*;
use serde_json::{from_str};

#[derive(Serialize, Deserialize)]
pub struct TradingPair { 
    pub mts: i64,
    pub amount: f64,   
    pub price: f64,                   
    pub rate: f64                  
}

#[derive(Serialize, Deserialize)]
pub struct FundingCurrency { 
    pub mts: i64,
    pub amount: f64,   
    pub price: f64,                   
    pub rate: f64,
    pub period: i64                  
}

#[derive(Clone)]
pub struct Trades {
    client: Client,
}

impl Trades {
    pub fn new() -> Self {
        Trades {
            client: Client::new(None, None),
        }
    }

    pub fn funding_currency<S>(&self, symbol: S) -> Result<(Vec<FundingCurrency>)>
        where S: Into<String>
    {     
        let endpoint: String = format!("trades/f{}/hist", symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let trades: Vec<FundingCurrency> = from_str(data.as_str()).unwrap();

        Ok(trades)
    }    

    pub fn trading_pair<S>(&self, symbol: S) -> Result<(Vec<TradingPair>)>
        where S: Into<String>
    {     
        let endpoint: String = format!("trades/t{}/hist", symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let trades: Vec<TradingPair> = from_str(data.as_str()).unwrap();

        Ok(trades)
    }
}