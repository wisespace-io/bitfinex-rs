use client::*;
use errors::*;
use serde_json::{from_str};

#[derive(Serialize, Deserialize)]
pub struct Order { 
    pub id: i64,   
    pub group_id: i32,                   
    pub client_id: i32,
    pub symbol: String,
    pub creation_timestamp: i32,
    pub update_timestamp: i32,
    pub amount: f64,
    pub amount_original: f64,
    pub order_type: String,
    pub previous_order_type: String,   
    pub flags: i32,                   
    pub order_status: String,
    pub price: f64,
    pub price_avg: f64,
    pub price_trailing: f64,
    pub price_aux_limit: f64,
    pub notify: i32,
    pub hidden: i32, 
    pub placed_id: i32                             
}

#[derive(Clone)]
pub struct Orders {
    client: Client,
}

impl Orders {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Orders {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn active_orders(&self, symbol: String) -> Result<(Vec<Order>)> {
        let endpoint: String = format!("auth/r/orders/:{}", symbol);
        let data = self.client.post_signed(endpoint, "orders".to_owned())?;

        let orders: Vec<Order> = from_str(data.as_str()).unwrap();

        Ok(orders)
    }
}