use client::*;
use errors::*;
use model::*;
use serde_json::{from_str, to_string};
use std::collections::BTreeMap;

static ORDER_TYPE_LIMIT: &str = "EXCHANGE LIMIT";

#[derive(Clone)]
pub struct Orders {
    client: Client,
}

struct OrderRequest {
    pub order_type: String,
    pub symbol: String,
    pub amount: f64,
    pub price: f64
}

impl Orders {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Orders {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn active_orders(&self) -> Result<Vec<Order>> {
        let payload: String = format!("{}", "{}");

        self.orders("orders".to_owned(), payload)
    }

    pub fn history<T>(&self, symbol: T) -> Result<Vec<Order>>
        where T: Into<Option<String>>
    {    
        let value = symbol.into().unwrap_or("".into());
        let payload: String = format!("{}", "{}");

        if value.is_empty() {
            return self.orders("orders/hist".into(), payload);
        } else {
            let request: String = format!("orders/t{}/hist", value);
            return self.orders(request, payload);
        }
    }

    pub fn orders<S>(&self, request: S, payload: S) -> Result<Vec<Order>>
        where S: Into<String>
    {    
        let data = self.client.post_signed(request.into(), payload.into())?;
        let orders: Vec<Order> = from_str(data.as_str())?;

        Ok(orders)
    }
    pub fn submit_order<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<TradeResponse>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy: OrderRequest = OrderRequest {
            order_type: ORDER_TYPE_LIMIT.to_string(),
            symbol: symbol.into(),
            amount: qty.into(),
            price,
        };

        let order = self.build_order(buy);
        let payload = to_string(&order)?;

        let data = self.client.post_signed_order("order/submit".into(), payload)?;
        println!("DATA: {:?}", data.as_str());
        let transaction: TradeResponse = from_str(data.as_str())?;

        println!("Trans: {:?}", transaction);
        Ok(transaction)
    }

    pub fn cancel_order(&self, order_id: i64) -> Result<TradeCancelResponse>
    {
        let mut parameters: BTreeMap<String, i64> = BTreeMap::new();
        parameters.insert("id".into(), order_id);
        let payload = to_string(&parameters)?;
        let data = self.client.post_signed_order("order/cancel".into(), payload)?;
        let order_canceled: TradeCancelResponse = from_str(data.as_str())?;

        Ok(order_canceled)
    }
    
    fn build_order(&self, order: OrderRequest) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), order.symbol);
        order_parameters.insert("type".into(), order.order_type);
        order_parameters.insert("amount".into(), order.amount.to_string());

        if order.price != 0.0 {
            order_parameters.insert("price".into(), order.price.to_string());
        }

        order_parameters
    }
}