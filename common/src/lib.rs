use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct PaymentEntry {
    pub price: f64,
    pub payment_method: String,
    pub goods_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDatas {
    pub payments: Vec<PaymentEntry>,
}

impl PaymentDatas {
    pub fn new() -> PaymentDatas {
        PaymentDatas {
            payments: Vec::new(),
        }
    }
}
