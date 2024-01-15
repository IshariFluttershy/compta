use chrono::{DateTime, Utc, Datelike};

use common::{PaymentEntry, PaymentMethod, GoodType, PaymentTotal, PaymentDatas};

pub fn calculate_payment_total(entries: &[&PaymentEntry]) -> PaymentTotal {
    PaymentTotal{
        total: calculate_total(entries),

        cb: calculate_total_for_payment_method(entries, PaymentMethod::CarteBleue),
        cash: calculate_total_for_payment_method(entries, PaymentMethod::Especes),
        
        food: calculate_total_for_good_type(entries, GoodType::Nourriture),
        charges: calculate_total_for_good_type(entries, GoodType::Charges),
        miscellaneous: calculate_total_for_good_type(entries, GoodType::Autres),
    
        cb_charges: calculate_combined_total(entries, PaymentMethod::CarteBleue, GoodType::Charges),
        cb_food: calculate_combined_total(entries, PaymentMethod::CarteBleue, GoodType::Nourriture),
        cb_miscellaneous: calculate_combined_total(entries, PaymentMethod::CarteBleue, GoodType::Autres),
        cash_charges: calculate_combined_total(entries, PaymentMethod::Especes, GoodType::Charges),
        cash_food: calculate_combined_total(entries, PaymentMethod::Especes, GoodType::Nourriture),
        cash_miscellaneous: calculate_combined_total(entries, PaymentMethod::Especes, GoodType::Autres),
    }
}

fn calculate_total(entries: &[&PaymentEntry]) -> f64 {
    entries.iter().map(|entry| entry.price).sum()
}

fn calculate_total_for_payment_method(entries: &[&PaymentEntry], method: PaymentMethod) -> f64 {
    entries.iter()
           .filter(|entry| entry.payment_method == method.as_str())
           .map(|entry| entry.price)
           .sum()
}

fn calculate_total_for_good_type(entries: &[&PaymentEntry], good_type: GoodType) -> f64 {
    entries.iter()
           .filter(|entry| entry.goods_type == good_type.as_str())
           .map(|entry| entry.price)
           .sum()
}

fn calculate_combined_total(entries: &[&PaymentEntry], method: PaymentMethod, good_type: GoodType) -> f64 {
    entries.iter()
           .filter(|entry| entry.payment_method == method.as_str() && entry.goods_type == good_type.as_str())
           .map(|entry| entry.price)
           .sum()
}

pub fn get_date_entries_readonly(data: &PaymentDatas, month: u32, year: u32) -> Vec<&PaymentEntry>
{
    data.payments.iter().filter(|entry| {
        let entry_date = DateTime::<Utc>::from_timestamp(entry.date, 0).unwrap().date_naive();
        (month == 0 || entry_date.month() == month) &&
        (year == 0 || entry_date.year_ce() == (true, year))
    }).collect()
}

pub fn get_date_entries(data: &PaymentDatas, month: u32, year: u32) -> PaymentDatas
{
    PaymentDatas { payments: data.payments.clone().into_iter().filter(|entry| {
        let entry_date = DateTime::<Utc>::from_timestamp(entry.date, 0).unwrap().date_naive();
        (month == 0 || entry_date.month() == month) &&
            (year == 0 || entry_date.year_ce() == (true, year))
    }).collect()
    }
}