use client::*;
use errors::*;
use serde_json::{from_str};

#[derive(Serialize, Deserialize)]
pub struct Order { 
    pub id: i64,   
    pub group_id: Option<i32>,                   
    pub client_id: i64,
    pub symbol: String,
    pub creation_timestamp: i64,
    pub update_timestamp: i64,
    pub amount: f64,
    pub amount_original: f64,
    pub order_type: String,
    pub previous_order_type: Option<String>,

    #[serde(skip_serializing)]
    pub placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    pub placeholder_2: Option<String>,
    
    pub flags: Option<i32>,                   
    pub order_status: Option<String>,

    #[serde(skip_serializing)]
    pub placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    pub placeholder_4: Option<String>,

    pub price: f64,
    pub price_avg: f64,
    pub price_trailing: Option<f64>,
    pub price_aux_limit: Option<f64>,
    
    #[serde(skip_serializing)]
    pub placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    pub placeholder_6: Option<String>,
    #[serde(skip_serializing)]
    pub placeholder_7: Option<String>,    
    
    pub notify: i32,
    pub hidden: i32,
    pub placed_id: Option<i32>                      
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

    pub fn active_orders(&self) -> Result<(Vec<Order>)> {
        let endpoint: String = format!("auth/r/orders");

        self.orders(endpoint, "orders".to_owned())
    }

    pub fn history<T>(&self, symbol: T) -> Result<(Vec<Order>)>
        where T: Into<Option<String>>
    {    
        let value = symbol.into().unwrap_or("".into());

        let mut endpoint: String = format!("auth/r/");

        if value.is_empty() {
            endpoint.push_str("orders/hist");
            return self.orders(endpoint, "orders/hist".to_owned());
        } else {
            let request: String = format!("orders/t{}/hist", value);
            endpoint.push_str(request.as_str());
            return self.orders(endpoint, request);
        }
    }

    pub fn orders<S>(&self, endpoint: S, request: S) -> Result<(Vec<Order>)>
        where S: Into<String>
    {    
        let data = self.client.post_signed(endpoint.into(), request.into())?;

        let orders: Vec<Order> = from_str(data.as_str()).unwrap();

        Ok(orders)
    }
}