mod env;
mod status;

use status::StatusKind;
use web_sys::{
    js_sys::{ArrayBuffer, Uint8Array},
    wasm_bindgen::{closure::Closure, JsCast},
    BinaryType, HtmlInputElement, MessageEvent, WebSocket,
};
use yew::{
    function_component, html, use_state_eq, Html, InputEvent, SubmitEvent, TargetCast,
    UseStateHandle,
};

fn on_input(
    text: UseStateHandle<String>,
    label: UseStateHandle<StatusKind>,
) -> impl Fn(InputEvent) {
    move |event: InputEvent| {
        if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
            text.set(input.value())
        } else {
            log::error!("{}", StatusKind::InvalidInput);
            label.set(StatusKind::InvalidInput)
        }
    }
}

fn on_submit(
    text: UseStateHandle<String>,
    label: UseStateHandle<StatusKind>,
) -> impl Fn(SubmitEvent) {
    move |event: SubmitEvent| {
        // prevent page reload
        event.prevent_default();

        // current text input
        let text = text.trim().to_string();

        // discontinue if the input is empty
        if text.is_empty() {
            return label.set(StatusKind::InvalidInput);
        }

        // only communicate with server if input exists
        let ws = match WebSocket::new(env::WLRS_WEBSOCKET_ADDR) {
            Ok(s) => s,
            Err(_) => {
                log::error!("{}", StatusKind::Connection);
                label.set(StatusKind::Connection);
                return;
            }
        };
        ws.set_binary_type(BinaryType::Arraybuffer);

        {
            let label = label.clone();
            let ws_onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
                let res = e
                    .data()
                    .dyn_into::<ArrayBuffer>()
                    .map(|buf| Uint8Array::new(&buf).to_vec())
                    .unwrap_or(vec![4]);

                let kind = StatusKind::from_u8(res.first().unwrap_or(&4));
                label.set(kind)
            }) as Box<dyn Fn(_)>);

            ws.set_onmessage(Some(ws_onmessage.as_ref().unchecked_ref()));
            ws_onmessage.forget();
        }

        {
            let label = label.clone();
            let ws_clone = ws.clone();
            let ws_onopen = Closure::wrap(Box::new(move || {
                label.set(StatusKind::Connecting);
                ws_clone.send_with_str(text.as_str()).unwrap();
            }) as Box<dyn Fn()>);
            ws.set_onopen(Some(ws_onopen.as_ref().unchecked_ref()));
            ws_onopen.forget();
        }

        {
            let label = label.clone();
            let ws_clone = ws.clone();
            let ws_onerror = Closure::wrap(Box::new(move || {
                label.set(StatusKind::Connection);
                ws_clone.close().unwrap_or_default();
            }) as Box<dyn Fn()>);
            ws.set_onerror(Some(ws_onerror.as_ref().unchecked_ref()));
            ws_onerror.forget();
        }
    }
}

#[allow(non_snake_case)]
#[function_component]
fn App() -> Html {
    let text = use_state_eq(String::new);
    let label_opt = use_state_eq(StatusKind::default);

    let oninput = on_input(text.clone(), label_opt.clone());
    let onsubmit = on_submit(text.clone(), label_opt.clone());

    html! {
        <div class="outer" action="#">
            <div class="inner">
                <form {onsubmit}>
                    <input {oninput} type="text" placeholder="Enter Minecraft Player Name"/>
                    <input type="submit"/>
                </form>
                if label_opt.is_new() {
                    { label_opt.as_html() }
                }
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}