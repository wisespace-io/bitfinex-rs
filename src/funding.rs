use client::*;
use errors::*;
use serde_aux::prelude::*;
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
pub struct ActiveOffer {
    pub id: i64,
    pub symbol: String,
    pub mts_created: i64,
    pub mts_updated: i64,
    pub amount: f64,
    pub amount_orig: f64,
    pub order_type: String,
    _placeholder_1: Option<String>,
    _placeholder_2: Option<String>,
    pub flags: Option<i8>,
    pub status: String,
    _placeholder_3: Option<String>,
    _placeholder_4: Option<String>,
    _placeholder_5: Option<String>,
    pub rate: f64,
    pub period: i64,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub notify: bool,
    pub hidden: i32,
    _placeholder_6: Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub renew: bool,
    _placeholder_7: Option<String>,
}

#[derive(Serialize)]
pub struct FundingOfferParams {
    #[serde(rename = "type")]
    pub t: String,
    pub symbol: String,
    pub amount: String,
    pub rate: String,
    pub period: i32,
}

#[derive(Serialize, Deserialize)]
pub struct FundingOffer {
    pub mts: i64,
    pub notification_type: String,
    pub message_id: Option<String>,
    null: Option<String>,
    pub offer: ActiveOffer,
    pub code: Option<String>,
    pub status: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct Currency {
    pub currency: String,
}

#[derive(Serialize, Deserialize)]
pub struct CancellAllFundingResponse {
    pub mts: i64,
    pub t: String,
    _placeholder_1: Option<String>,
    _placeholder_2: Option<String>,
    _placeholder_3: Option<String>,
    _placeholder_4: Option<String>,
    pub status: String,
    pub text: String,
}

#[derive(Clone)]
pub struct Funding {
    client: Client,
}

impl Funding {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Funding {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn get_active_offers(&self, symbol: String) -> Result<Vec<ActiveOffer>> {
        let payload: String = format!("{}", "");

        let data = self
            .client
            .post_signed(format!("r/funding/offers/{}", symbol).into(), payload)?;
        let funding_offers: Vec<ActiveOffer> = from_str(data.as_str())?;
        Ok(funding_offers)
    }

    pub fn submit_funding_offer(&self, params: FundingOfferParams) -> Result<FundingOffer> {
        let payload = serde_json::to_string(&params).unwrap();
        let data = self
            .client
            .post_signed("w/funding/offer/submit".to_string(), payload)?;
        let offer: FundingOffer = from_str(data.as_str())?;

        Ok(offer)
    }

    pub fn cancel_all_funding_offers(&self, symbol: String) -> Result<CancellAllFundingResponse> {
        let params = Currency { currency: symbol };
        let payload = serde_json::to_string(&params).unwrap();
        let data = self
            .client
            .post_signed("w/funding/offer/cancel/all".to_string(), payload)?;
        let res: CancellAllFundingResponse = from_str(data.as_str())?;

        Ok(res)
    }
}
