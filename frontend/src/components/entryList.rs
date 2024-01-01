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
            <p>{format!("Nº{} : Prix : {}, Méthode de paiement : {}, Type d'achat : {}", i+1, entry.price, entry.payment_method, entry.goods_type)}</p>
        });
    }

    result
}