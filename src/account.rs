use client::*;
use errors::*;
use serde_json::{from_str};

#[derive(Serialize, Deserialize)]
pub struct Wallet { 
    pub wallet_type: String,   
    pub currency: String,                   
    pub balance: f64,
    pub unsettled_interest: f64,
    pub balance_available: Option<f64>             
}

#[derive(Clone)]
pub struct Account {
    client: Client,
}

impl Account {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Account {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn get_wallets(&self) -> Result<(Vec<Wallet>)> {
        let endpoint: String = format!("auth/r/wallets");

        let data = self.client.post_signed(endpoint.into(), "wallets".into())?;

        let wallets: Vec<Wallet> = from_str(data.as_str()).unwrap();

        Ok(wallets)
    }
}