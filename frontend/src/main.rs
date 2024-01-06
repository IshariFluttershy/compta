use chrono::{NaiveDate, NaiveDateTime};
use common::{PaymentDatas, PaymentTotal};
use gloo_console::log;
use reqwasm::http::*;
use wasm_bindgen::JsCast;
use web_sys::{console::log, EventTarget, HtmlInputElement, HtmlSelectElement};
use yew::{platform::spawn_local, prelude::*};

use crate::components::{entry_list::EntryList, total::Total};

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
    let month: UseStateHandle<u32> = use_state(|| 0);
    let year: UseStateHandle<u32> = use_state(|| 0);
    let goods_type_handle: UseStateHandle<String> = use_state(|| String::from("Nourriture"));
    let payment_type_handle: UseStateHandle<String> = use_state(|| String::from("Carte bleue"));
    let date_handle: UseStateHandle<i64> = use_state(|| chrono::Local::now().timestamp());
    let payment_data_vec: UseStateHandle<PaymentDatas> = use_state(PaymentDatas::new);
    let payment_total: UseStateHandle<PaymentTotal> = use_state(PaymentTotal::new);

    let payment_data_vec_clone = payment_data_vec.clone();
    let payment_total_clone = payment_total.clone();
    let month_clone1 = month.clone();
    let year_clone1 = year.clone();
    let month_clone2 = month.clone();
    let year_clone2 = year.clone();

    let get_data = move || {
        spawn_local(async move {
            match Request::get(
                &format!(
                    "/get_data?month={}&year={}",
                    *month_clone1, *year_clone1,
                ))
            .send().await {
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

    let get_total = move || {
        spawn_local(async move {
            match Request::get(                &format!(
                "/get_total?month={}&year={}",
                *month_clone2, *year_clone2,
            ))
            .send().await {
                Ok(data) => match data.json::<PaymentTotal>().await {
                    Ok(data) => {
                        log!("success2");
                        payment_total_clone.set(data);
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
    use_state(get_total.clone());

    let on_add_payment_click = {
        let price = price.clone();
        let goods_type_handle = goods_type_handle.clone();
        let payment_type_handle = payment_type_handle.clone();
        let date_handle = date_handle.clone();
        let get_data = get_data.clone();
        let get_total = get_total.clone();


        move |_| {
            let price = price.clone();
            let goods_type_handle = goods_type_handle.clone();
            let payment_type_handle = payment_type_handle.clone();
            let date_handle = date_handle.clone();
            let get_data = get_data.clone();
            let get_total = get_total.clone();


            spawn_local(async move {
                match Request::post("/command")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(wasm_bindgen::JsValue::from_str(&format!(
                        "price={}&goods_type={}&payment_method={}&date={}",
                        *price, *goods_type_handle, *payment_type_handle, *date_handle,
                    )))
                    .send()
                    .await
                {
                    Ok(entries1) => match entries1.ok() {
                        true => {
                            log!("success");
                            get_data();
                            get_total();
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
        let get_total = get_total.clone();


        move |_| {
            let id_to_delete = id_to_delete.clone();
            let get_data = get_data.clone();
            let get_total = get_total.clone();


            spawn_local(async move {
                let resp = Request::post("/delete")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(wasm_bindgen::JsValue::from_str(&format!(
                        "id={}",
                        *id_to_delete - 1
                    )))
                    .send()
                    .await
                    .unwrap();

                get_data();
                get_total();
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

    let on_month_input_change = {
        let month: UseStateHandle<u32> = month.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                month.set(input.value().parse::<u32>().unwrap());
            }
        })
    };

    let on_year_input_change = {
        let year = year.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                year.set(input.value().parse::<u32>().unwrap());
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

    let on_date_change = {
        let date_handle = date_handle.clone();
        log!("date change");

        Callback::from(move |e: Event| {
            log!("date change callback");

            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                log!("some input = ", input.value());

                date_handle.set(
                    NaiveDate::parse_from_str(input.value().as_str(), "%Y-%m-%d")
                        .unwrap()
                        .and_hms_opt(9, 10, 11)
                        .unwrap()
                        .timestamp(),
                );
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
                <input type="date" id="buy_date" name="buy_date" value={NaiveDateTime::from_timestamp_opt(*date_handle, 0).unwrap().date().to_string()} min="2023-01-01" max="2025-12-31" onchange={on_date_change}/>
                <button onclick={on_add_payment_click}>{ "Valider" }</button>
            </p>
            <p>
                <input type="number" id="IdToDelete" name="IdToDelete" placeholder="Numéro du payement a supprimer" onchange={on_delete_input_change}/>
                <button onclick={on_delete_payment_click}>{ "Supprime le paiement" }</button>
            </p>
            <p>
                <input type="number" id="Month" name="Month" placeholder="1" onchange={on_month_input_change}/>
                <input type="number" id="Year" name="Year" placeholder="2023" onchange={on_year_input_change}/>
            </p>
            <p>
                <EntryList entries={payment_data_vec.payments.clone()} />
                <Total total={(*payment_total).clone()} />
            </p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
