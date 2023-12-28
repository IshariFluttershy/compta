use common::PaymentEntry;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EntryListProps {
    pub entries: Vec<PaymentEntry>,
}

#[function_component(EntryList)]
pub fn entry_list(EntryListProps { entries }: &EntryListProps) -> Html {
    entries.iter().map(|entry| html! {
        <p>{format!("{}: {}, {}", entry.price, entry.payment_method, entry.goods_type)}</p>
    }).collect()
}