use client::*;
use errors::*;
use serde_json::{from_str};

#[derive(Serialize, Deserialize)]
pub struct Candle { 
    pub timestamp: i64,   
    pub open: f64,                   
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64                        
}

#[derive(Clone)]
pub struct Candles {
    client: Client,
}

impl Candles {
    pub fn new() -> Self {
        Candles {
            client: Client::new(None, None),
        }
    }

    pub fn last(&self, symbol: String, timeframe: String) -> Result<(Candle)> {
        let endpoint: String = format!("candles/trade:{}:t{}/last", timeframe, symbol);
        let data = self.client.get(endpoint, String::new())?;

        let history: Candle = from_str(data.as_str()).unwrap();

        Ok(history)
    }    

    pub fn history(&self, symbol: String, timeframe: String) -> Result<(Vec<Candle>)> {
        let endpoint: String = format!("candles/trade:{}:t{}/hist", timeframe, symbol);
        let data = self.client.get(endpoint, String::new())?;

        let history: Vec<Candle> = from_str(data.as_str()).unwrap();

        Ok(history)
    }
}