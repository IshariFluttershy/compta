use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub payment_method: String,
    pub goods_type: String,
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <input type="text" id="Name" name="Name" placeholder="Prix"/>
            <select name="model" id="model-select" onchange={props.payment_method}>
                <option value="stable_diffusion_onnx" selected={true}>{"Stable Diffusion"}</option>
                <option value="waifu-diffusion-diffusers-onnx-v1-3">{"Waifu Diffusion"}</option>
                <option value="hassanblend_onnx">{"Hassanblend"}</option>
            </select>
            <select name="model" id="model-select" onchange={goods_type}>
                <option value="stable_diffusion_onnx" selected={true}>{"Stable Diffusion"}</option>
                <option value="waifu-diffusion-diffusers-onnx-v1-3">{"Waifu Diffusion"}</option>
                <option value="hassanblend_onnx">{"Hassanblend"}</option>
            </select>
            <button {onclick}>{ "Valider" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}