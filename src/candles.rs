use client::*;
use errors::*;
use serde_json::from_str;

#[derive(Debug, Clone, Default)]
pub struct CandleHistoryParams {
    /// Number of candles requested (Max: 10000)
    pub limit: Option<i32>,

    /// Filter start (ms)
    pub start: Option<i64>,

    /// Filter end (ms)
    pub end: Option<i64>,

    /// Sorts the results from old > new
    pub sort: Option<bool>,
}

impl CandleHistoryParams {
    pub fn new() -> Self {
        Self {
            limit: Some(120),
            sort: Some(false),
            start: None,
            end: None,
        }
    }

    pub fn to_query(&self) -> String {
        format!("{}={}&{}={}&{}={}&{}={}",
            "limit", self.limit
                         .map(|a| a.to_string())
                         .unwrap_or("".into()),
            "start", self.start
                         .map(|a| a.to_string())
                         .unwrap_or("".into()),
            "end", self.end
                       .map(|a| a.to_string())
                       .unwrap_or("".into()),
            "sort", self.sort
                        .map(|a| if a { "1" } else { "0" })
                        .unwrap_or("".into()),
        )
    }
}

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

    pub fn last<S>(&self, symbol: S, timeframe: S) -> Result<Candle>
        where S: Into<String>
    {    
        let endpoint: String = format!("candles/trade:{}:t{}/last", timeframe.into(), symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let history: Candle = from_str(data.as_str())?;

        Ok(history)
    }    

    pub fn history<S>(
        &self,
        symbol: S,
        timeframe: S,
        params: &CandleHistoryParams,
    ) -> Result<Vec<Candle>>
        where S: Into<String>
    {    
        let endpoint: String = format!("candles/trade:{}:t{}/hist", timeframe.into(), symbol.into());
        let data = self.client.get(endpoint, params.to_query())?;

        let history: Vec<Candle> = from_str(data.as_str())?;

        Ok(history)
    }
}