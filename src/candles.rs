use client::*;
use errors::*;
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
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

    pub fn last<S>(&self, symbol: S, timeframe: S) -> Result<(Candle)>
        where S: Into<String>
    {    
        let endpoint: String = format!("candles/trade:{}:t{}/last", timeframe.into(), symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let history: Candle = from_str(data.as_str())?;

        Ok(history)
    }    

    pub fn history<S>(&self, symbol: S, timeframe: S) -> Result<(Vec<Candle>)>
        where S: Into<String>
    {    
        let endpoint: String = format!("candles/trade:{}:t{}/hist", timeframe.into(), symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let history: Vec<Candle> = from_str(data.as_str())?;

        Ok(history)
    }
}