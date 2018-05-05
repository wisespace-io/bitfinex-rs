use ticker::*;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum NotificationEvent {
    Info { event: String, version: u16, platform: Platform },

    #[serde(rename_all = "camelCase")]
    TickerTradingSubsbribed { event: String, channel: String, chan_id: u16, symbol: String, pair: String },

    #[serde(rename_all = "camelCase")]
    TickerFundingSubsbribed { event: String, channel: String, chan_id: u16, symbol: String, currency: String },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DataEvent {
    TickerTradingEvent (i32, TradingPair),
    TickerFundingEvent (i32, FundingCurrency),
    HeartbeatEvent (i32, String)
}

#[derive(Debug, Deserialize)]
pub struct Platform {
    pub status: u16,
}

