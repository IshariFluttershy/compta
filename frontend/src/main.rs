use common::PaymentEntry;
use web_sys::{EventTarget, HtmlSelectElement, HtmlInputElement};
use yew::{prelude::*, html::IntoPropValue, platform::spawn_local};
use wasm_bindgen::JsCast;
use reqwasm::http::*;
use gloo_console::log;

use crate::components::entryList::EntryList;

mod components;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub payment_method: String,
    pub goods_type: String,
}

#[function_component]
fn App() -> Html {
    let price: UseStateHandle<f64> = use_state(|| 0.);
    let id_to_delete: UseStateHandle<usize> = use_state(|| 0);

    let goods_type_handle: UseStateHandle<String> = use_state(String::default);
    let payment_type_handle: UseStateHandle<String> = use_state(String::default);
    let payment_data_handle: UseStateHandle<String> = use_state(String::default);

    let payment_data_vec: UseStateHandle<Vec<PaymentEntry>> = use_state(Vec::default);

    let on_add_payment_click = {
        let price = price.clone();
        let goods_type_handle = goods_type_handle.clone();
        let payment_type_handle = payment_type_handle.clone();
        let payment_data_handle = payment_data_handle.clone();
        let payment_data_vec = payment_data_vec.clone();


        move |_| {
            let price = price.clone();
            let goods_type_handle = goods_type_handle.clone();
            let payment_type_handle = payment_type_handle.clone();
            let payment_data_handle = payment_data_handle.clone();
            let payment_data_vec = payment_data_vec.clone();


            spawn_local(async move {
                let resp = Request::post("/command")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(wasm_bindgen::JsValue::from_str(
                        &format!("price={}&goods_type={}&payment_type={}",
                        *price,
                        *goods_type_handle,
                        *payment_type_handle,
                        )))
                    .send()
                    .await
                    .unwrap();

                payment_data_handle.set(resp.text().await.unwrap());
                let test = PaymentEntry{
                    price : *price,
                    goods_type : (*goods_type_handle).clone(),
                    payment_method : (*payment_type_handle).clone(),
                };
                log!("avant = {}", payment_data_vec.len());
                let mut cloned: Vec<_> = payment_data_vec.to_vec();
                cloned.push(test);
                payment_data_vec.set(cloned);
                log!("apres = {}", payment_data_vec.len());
            });
        }
    };

    let on_delete_payment_click = {
        let id_to_delete = id_to_delete.clone();
        let payment_data_handle = payment_data_handle.clone();

        move |_| {
            let id_to_delete = id_to_delete.clone();
            let payment_data_handle = payment_data_handle.clone();

            spawn_local(async move {
                let resp = Request::post("/delete")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(wasm_bindgen::JsValue::from_str(
                        &format!("id={}",
                        *id_to_delete
                        )))
                    .send()
                    .await
                    .unwrap();

                payment_data_handle.set(resp.text().await.unwrap());
            });
        }
    };

    let on_price_input_change = {
        let price = price.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                price.set(input.value().parse::<f64>().unwrap());
            }
        })
    };

    let on_delete_input_change = {
        let id_to_delete = id_to_delete.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                id_to_delete.set(input.value().parse::<usize>().unwrap());
            }
        })
    };

    let on_goods_type_change = {
        let goods_type_handle = goods_type_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                goods_type_handle.set(input.value());
            }
        })
    };

    let on_payment_type_change = {
        let payment_type_handle = payment_type_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                payment_type_handle.set(input.value());
            }
        })
    };

    html! {
        <div>
            <p>
                <input type="number" id="Price" name="Price" placeholder="Prix" onchange={on_price_input_change}/>
                <select name="model" id="model-select" onchange={on_goods_type_change}>
                    <option value="Nourriture" selected={true}>{"Nourriture"}</option>
                    <option value="Charges">{"Charges"}</option>
                    <option value="Autres">{"Autres"}</option>
                </select>
                <select name="model" id="model-select" onchange={on_payment_type_change}>
                    <option value="Carte bleue" selected={true}>{"Carte bleue"}</option>
                    <option value="Especes">{"Especes"}</option>
                </select>
                <button onclick={on_add_payment_click}>{ "Valider" }</button>
            </p>
            <p>
                <input type="number" id="IdToDelete" name="IdToDelete" placeholder="Numéro du payement a supprimer" onchange={on_delete_input_change}/>
                <button onclick={on_delete_payment_click}>{ "Delete first entry" }</button>
            </p>

            <p>{ (*payment_data_handle).clone() }</p>
            <p>
                {"TEXTTTTETETT"}
            </p>
            <p>
                {"hehehe"}
                    <EntryList entries={(*payment_data_vec).clone()} />
                {"hehehe222222"}
            </p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}