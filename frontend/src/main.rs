use common::PaymentDatas;
use web_sys::{EventTarget, HtmlSelectElement, HtmlInputElement};
use yew::{prelude::*, platform::spawn_local};
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
    let id_to_delete: UseStateHandle<usize> = use_state(|| 1);
    let goods_type_handle: UseStateHandle<String> = use_state(|| {String::from("Nourriture")});
    let payment_type_handle: UseStateHandle<String> = use_state(|| {String::from("Carte bleue")});
    let payment_data_vec: UseStateHandle<PaymentDatas> = use_state(PaymentDatas::new);
    let payment_data_vec_clone = payment_data_vec.clone();

    let get_data = move || {
        spawn_local(async move {
        match Request::get("/get_data").send().await {
            Ok(data) => match data.json::<PaymentDatas>().await {
                Ok(data) => {
                    log!("success2");
                    payment_data_vec_clone.set(data);
                    true
                }
                Err(err) => {
                    log!("error 1 : ", data.url());
                    log!("error 1 : ", data.as_raw());
                    log!("error 1 : ", data.status_text());
                    log!("error 3 : ", err.to_string());
                    false
                }
            },
            Err(err) => {
                log!("error 4 : ", err.to_string());
                false
            }
        };
    });
};    
use_state(get_data.clone());


    let on_add_payment_click = {
        let price = price.clone();
        let goods_type_handle = goods_type_handle.clone();
        let payment_type_handle = payment_type_handle.clone();
        let get_data = get_data.clone();


        move |_| {
            let price = price.clone();
            let goods_type_handle = goods_type_handle.clone();
            let payment_type_handle = payment_type_handle.clone();
            let get_data = get_data.clone();



            spawn_local(async move {
                match Request::post("/command")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(wasm_bindgen::JsValue::from_str(
                        &format!("price={}&goods_type={}&payment_method={}",
                        *price,
                        *goods_type_handle,
                        *payment_type_handle,
                        )))
                    .send()
                    .await
                    {
                        Ok(entries1) => match entries1.ok() {
                            true => {
                                log!("success");
                                get_data();
                            }
                            false => {
                                
                                log!("error 1 : ", entries1.url());
                                log!("error 1 : ", entries1.as_raw());
                                log!("error 1 : ", entries1.status_text());
                            }
                        },
                        Err(err) => {
                            log!("error 2 : ", err.to_string());
                        }
                    };
            });
        }
    };

    let on_delete_payment_click = {
        let id_to_delete = id_to_delete.clone();
        let get_data = get_data.clone();

        move |_| {
            let id_to_delete = id_to_delete.clone();
            let get_data = get_data.clone();

            spawn_local(async move {
                let resp = Request::post("/delete")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(wasm_bindgen::JsValue::from_str(
                        &format!("id={}",
                        *id_to_delete-1
                        )))
                    .send()
                    .await
                    .unwrap();

                get_data();
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
                <button onclick={on_delete_payment_click}>{ "Supprime le paiement" }</button>
            </p>
            <p>
                <EntryList entries={payment_data_vec.payments.clone()} />
            </p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}