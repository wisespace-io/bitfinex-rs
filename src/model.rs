use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
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
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    
    pub flags: Option<i32>,                   
    pub order_status: Option<String>,

    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,

    pub price: f64,
    pub price_avg: f64,
    pub price_trailing: Option<f64>,
    pub price_aux_limit: Option<f64>,
    
    #[serde(skip_serializing)]
    __placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,    
    
    pub notify: i32,
    pub hidden: i32,
    pub placed_id: Option<i32>                      
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade { 
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
    pub time_in_forse: Option<i64>,

    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    
    pub flags: Option<i32>,                   
    pub order_status: String,

    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    
    pub price: f64,
    pub price_avg: f64,
    pub price_trailing: Option<f64>,
    pub price_aux_limit: Option<f64>,
    
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,
      
    pub hidden: i32,
    pub placed_id: Option<i32>,

    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,  
    #[serde(skip_serializing)]
    _placeholder_8: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_9: Option<String>,
    
    routing: Option<String>,   

    #[serde(skip_serializing)]
    _placeholder_10: Option<String>,    
    #[serde(skip_serializing)]
    _placeholder_11: Option<String>,
    
    meta: Option<String>,  
}


#[derive(Debug, Deserialize, Serialize)]
pub struct TradeResponse {
    pub millisecond_timestamp: i64, 
    pub req_type: String, 
    
    pub message_id: Option<i32>, 
    
    #[serde(skip_serializing)]
    pub _null: Option<()>, 
    
    pub order: Vec<Trade>,

    pub code: Option<i64>,
    pub status: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeCancelResponse {
    pub millisecond_timestamp: i64, 
    pub req_type: String, 
    
    pub message_id: Option<i32>, 
    
    #[serde(skip_serializing)]
    pub _null: Option<()>, 
    
    pub order: Trade,

    pub code: Option<i64>,
    pub status: String,
    pub text: String,
}

