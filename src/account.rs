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

#[derive(Serialize, Deserialize)]
pub struct MarginBase { 
    key: String,
    pub margin: Base
}

#[derive(Serialize, Deserialize)]
pub struct Base {
    pub user_profit_loss: f64,
    pub user_swaps: f64,      
    pub margin_balance: f64,
    pub margin_net: f64        
}

#[derive(Serialize, Deserialize)]
pub struct MarginSymbol { 
    key: String,
    symbol: String,    
    pub margin: Symbol
}

#[derive(Serialize, Deserialize)]
pub struct Symbol {
    pub tradable_balance: f64,
    pub gross_balance: f64,
    pub buy: f64,
    pub sell: f64,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct FundingInfo { 
    key: String,
    symbol: String,    
    pub funding: Funding
}

#[derive(Serialize, Deserialize)]
pub struct Funding {
    pub yield_loan: f64,
    pub yield_lend: f64,
    pub duration_loan: f64,
    pub duration_lend: f64
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
        let payload: String = format!("{}", "{}");
        let data = self.client.post_signed("wallets".into(), payload)?;

        let wallets: Vec<Wallet> = from_str(data.as_str())?;

        Ok(wallets)
    }

    pub fn margin_base(&self) -> Result<(MarginBase)>
    { 
        let payload: String = format!("{}", "{}");

        let data = self.client.post_signed("info/margin/base".into(), payload)?;

        let margin: MarginBase = from_str(data.as_str())?;

        Ok(margin)
    }

    pub fn margin_symbol<S>(&self, key: S) -> Result<(MarginSymbol)> 
        where S: Into<String>
    { 
        let payload: String = format!("{}", "{}");
        let request: String = format!("info/margin/t{}", key.into());

        let data = self.client.post_signed(request, payload)?;

        let margin: MarginSymbol = from_str(data.as_str())?;

        Ok(margin)
    }    

    pub fn funding_info<S>(&self, key: S) -> Result<(FundingInfo)> 
        where S: Into<String>
    { 
        let payload: String = format!("{}", "{}");
        let request: String = format!("info/funding/f{}", key.into());

        let data = self.client.post_signed(request, payload)?;

        let info: FundingInfo = from_str(data.as_str())?;

        Ok(info)
    }    
}