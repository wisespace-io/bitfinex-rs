use client::*;
use errors::*;
use serde_json::{from_str};

#[derive(Serialize, Deserialize)]
pub struct TradingPair { 
    pub bid: f64,
    pub bid_size: f64,   
    pub ask: f64,                   
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64                    
}

#[derive(Serialize, Deserialize)]
pub struct FundingCurrency { 
    pub frr: f64,
    pub bid: f64,
    pub bid_size: f64,   
    pub bid_period: i64,
    pub ask: f64,      
    pub ask_period: i64,              
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64                    
}

#[derive(Clone)]
pub struct Ticker {
    client: Client,
}

impl Ticker {
    pub fn new() -> Self {
        Ticker {
            client: Client::new(None, None),
        }
    }

    pub fn funding_currency(&self, symbol: String) -> Result<(FundingCurrency)> {
        let data = self.client.get("ticker/f", symbol)?;

        let ticker: FundingCurrency = from_str(data.as_str()).unwrap();

        Ok(ticker)
    }    

    pub fn trading_pair(&self, symbol: String) -> Result<(TradingPair)> {
        let data = self.client.get("ticker/t", symbol)?;

        let ticker: TradingPair = from_str(data.as_str()).unwrap();

        Ok(ticker)
    }
}