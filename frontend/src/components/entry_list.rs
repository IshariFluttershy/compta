use chrono::NaiveDateTime;
use common::PaymentEntry;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct EntryListProps {
    pub entries: Vec<PaymentEntry>,
    pub delete_callback: Callback<u128>,
}

#[function_component(EntryList)]
pub fn entry_list(EntryListProps { entries, delete_callback }: &EntryListProps) -> Html {
    let mut result: VNode = Default::default(); 
    for (i, entry) in entries.iter().enumerate() {
        let entry_id = entry.id;
        let delete = delete_callback.reform(move |_| entry_id); 
        result.to_vlist_mut().push(html! {
            <p>{format!("{} - {} - {} - {}", 
            NaiveDateTime::from_timestamp_opt(entry.date, 0).unwrap().date().format("%d/%m/%Y"),
            entry.payment_method,
            entry.goods_type,
            entry.price,
            )}
            <button onclick={delete} >
                {"Supprimer"}
            </button>
            </p>
        });
    }

    result
}