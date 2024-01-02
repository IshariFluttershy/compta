use chrono::NaiveDateTime;
use common::PaymentEntry;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct EntryListProps {
    pub entries: Vec<PaymentEntry>,
}

#[function_component(EntryList)]
pub fn entry_list(EntryListProps { entries }: &EntryListProps) -> Html {
    let mut result: VNode = Default::default(); 
    for (i, entry) in entries.iter().enumerate() {
        result.to_vlist_mut().push(html! {
            <p>{format!("Nº{} : {} - {} - {} - {}", 
            i+1,
            NaiveDateTime::from_timestamp_opt(entry.date, 0).unwrap().date().format("%d/%m/%Y"),
            entry.payment_method,
            entry.goods_type,
            entry.price,
            )}
            </p>
        });
    }

    result
}