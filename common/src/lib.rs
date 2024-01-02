use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct PaymentEntry {
    pub price: f64,
    pub payment_method: String,
    pub goods_type: String,
    pub date: i64
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaymentTotal {
    pub total: f64,

    pub cb: f64,
    pub cash: f64,

    pub food: f64,
    pub charges: f64,
    pub miscellaneous: f64,

    pub cb_food: f64,
    pub cb_charges: f64,
    pub cb_miscellaneous: f64,
    pub cash_food: f64,
    pub cash_charges: f64,
    pub cash_miscellaneous: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDatas {
    pub payments: Vec<PaymentEntry>,
}

impl PaymentTotal {
    pub fn new() -> PaymentTotal {
        PaymentTotal {
            total:0.,
            cb:0.,
            cash:0.,
            food:0.,
            charges:0.,
            miscellaneous:0.,
            cb_charges:0.,
            cb_food:0.,
            cb_miscellaneous:0.,
            cash_charges:0.,
            cash_food:0.,
            cash_miscellaneous:0.,
        }
    }
}

impl Default for PaymentTotal {
    fn default() -> Self {
        Self::new()
    }
}

impl PaymentDatas {
    pub fn new() -> PaymentDatas {
        PaymentDatas {
            payments: Vec::new(),
        }
    }
}

impl Default for PaymentDatas {
    fn default() -> Self {
        Self::new()
    }
}
