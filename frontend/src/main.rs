use web_sys::{EventTarget, HtmlSelectElement};
use yew::{prelude::*, html::IntoPropValue, platform::spawn_local};
use wasm_bindgen::JsCast;
use reqwasm::http::*;
use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub payment_method: String,
    pub goods_type: String,
}

#[function_component]
fn App() -> Html {
    let price = use_state(|| 0.);

    /*let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let test = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };*/

    let goods_type_handle = use_state(String::default);
    let payment_type_handle = use_state(String::default);

    let payment_data_handle = use_state(String::default);


    let onclick = {
        let price = price.clone();
        let goods_type_handle = goods_type_handle.clone();
        let payment_type_handle = payment_type_handle.clone();
        let payment_data_handle = payment_data_handle.clone();


        move |_| {
            let price = price.clone();
            let goods_type_handle = goods_type_handle.clone();
            let payment_type_handle = payment_type_handle.clone();
            let payment_data_handle = payment_data_handle.clone();


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

                //log!(resp.text().await.unwrap());
                payment_data_handle.set(resp.text().await.unwrap());
            });
        }
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
            <input type="text" id="Name" name="Name" placeholder="Prix"/>
            <select name="model" id="model-select" onchange={on_goods_type_change}>
                <option value="Nourriture" selected={true}>{"Nourriture"}</option>
                <option value="Charges">{"Charges"}</option>
                <option value="Autres">{"Autres"}</option>
            </select>
            <select name="model" id="model-select" onchange={on_payment_type_change}>
                <option value="Carte bleue" selected={true}>{"Carte bleue"}</option>
                <option value="Especes">{"Especes"}</option>
            </select>
            <button {onclick}>{ "Valider" }</button>
            <p>{ (*payment_data_handle).clone() }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}