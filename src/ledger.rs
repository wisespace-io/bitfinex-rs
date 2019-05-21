use client::*;
use errors::*;
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub id: i64,
    pub currency: String,
    _field3: Option<()>,
    pub timestamp_milli: i64,
    _field5: Option<()>,
    pub amount: f64,
    pub balance: f64,
    _field8: Option<()>,
    pub description: String,
}

#[derive(Clone)]
pub struct Ledger {
    client: Client,
}

impl Ledger {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Ledger {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn get_history<S>(
        &self,
        symbol: S,
        _start: u128,
        _end: u128,
        _limit: i32,
    ) -> Result<(Vec<Entry>)>
    where
        S: Into<String>,
    {
        let payload: String = format!("{}", "{}");
        let request: String = format!("ledgers/{}/hist", symbol.into());

        // TODO: Provide query parameters. These must not be signed.
        let data = self.client.post_signed(request, payload)?;

        let entry: Vec<Entry> = from_str(data.as_str())?;

        Ok(entry)
    }
}
