use yew::prelude::*;
use web_sys::{HtmlSelectElement, HtmlInputElement, Event, InputEvent};
use wasm_bindgen::prelude::*;
use crate::{generate_product_key, validate_product_key};

struct App {
    key_type: String,
    generated_key: String,
    key_to_validate: String,
    validation_result: String,
}

enum Msg {
    GenerateKey,
    ValidateKey,
    UpdateKeyType(String),
    UpdateKeyToValidate(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            key_type: "retail".to_string(),
            generated_key: "".to_string(),
            key_to_validate: "".to_string(),
            validation_result: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GenerateKey => {
                self.generated_key = generate_product_key(&self.key_type);
                true
            }
            Msg::ValidateKey => {
                self.validation_result = if validate_product_key(&self.key_to_validate) {
                    "Key is valid ✅".to_string()
                } else {
                    "Key is invalid ❌".to_string()
                };
                true
            }
            Msg::UpdateKeyType(key_type) => {
                self.key_type = key_type;
                true
            }
            Msg::UpdateKeyToValidate(key) => {
                self.key_to_validate = key;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <h1>{ "keyforge95 Key Generator and Validator" }</h1>
                <div>
                    <h2>{ "Generate Product Key" }</h2>
                    <select onchange={link.callback(|e: Event| {
                        let input: HtmlSelectElement = e.target_unchecked_into();
                        Msg::UpdateKeyType(input.value())
                    })}>
                        <option value="retail">{ "Retail" }</option>
                        <option value="oem">{ "OEM" }</option>
                    </select>
                    <button onclick={link.callback(|_| Msg::GenerateKey)}>{ "Generate Key" }</button>
                    <p>{ &self.generated_key }</p>
                </div>
                <div>
                    <h2>{ "Validate Product Key" }</h2>
                    <input type="text" value={self.key_to_validate.clone()} oninput={link.callback(|e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Msg::UpdateKeyToValidate(input.value())
                    })} />
                    <button onclick={link.callback(|_| Msg::ValidateKey)}>{ "Validate Key" }</button>
                    <p>{ &self.validation_result }</p>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}