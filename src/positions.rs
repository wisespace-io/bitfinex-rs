#[derive(Debug, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub status: String,
    pub amount: f64,
    pub base_price: f64,
    pub margin_funding: f64,
    pub margin_funding_type: u8,
    pub pl: f64,
    pub pl_perc: f64,
    pub price_liq: f64,
    pub leverage: f64,
}
