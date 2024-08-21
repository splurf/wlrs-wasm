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
            log::error!("{}", StatusKind::InvalidInput.as_str());
            label.set(StatusKind::InvalidInput)
        }
    }
}

fn on_submit(
    user: UseStateHandle<String>,
    pass: UseStateHandle<String>,
    label: UseStateHandle<StatusKind>,
) -> impl Fn(SubmitEvent) {
    move |event: SubmitEvent| {
        // prevent page reload
        event.prevent_default();

        // current text input
        let user = user.trim();
        let pass = pass.trim();

        // discontinue if the input is empty
        if user.is_empty() || pass.is_empty() {
            return label.set(StatusKind::InvalidInput);
        }

        // only communicate with server if input exists
        let ws = match WebSocket::new(env::WEBSOCKET_ADDR) {
            Ok(s) => s,
            Err(_) => {
                log::error!("{}", StatusKind::Connection.as_str());
                return label.set(StatusKind::Connection);
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
            let user = user.to_owned();
            let pass = pass.to_owned();
            let label = label.clone();
            let ws_clone = ws.clone();
            let ws_onopen = Closure::wrap(Box::new(move || {
                label.set(StatusKind::Connecting);
                let data = [
                    [user.len() as u8].as_slice(),
                    user.as_bytes(),
                    pass.as_bytes(),
                ]
                .concat();
                _ = ws_clone.send_with_u8_array(data.as_slice());
            }) as Box<dyn Fn()>);
            ws.set_onopen(Some(ws_onopen.as_ref().unchecked_ref()));
            ws_onopen.forget();
        }

        {
            let label = label.clone();
            let ws_clone = ws.clone();
            let ws_onerror = Closure::wrap(Box::new(move || {
                label.set(StatusKind::Connection);
                _ = ws_clone.close()
            }) as Box<dyn Fn()>);
            ws.set_onerror(Some(ws_onerror.as_ref().unchecked_ref()));
            ws_onerror.forget();
        }
    }
}

#[allow(non_snake_case)]
#[function_component]
fn App() -> Html {
    let user = use_state_eq(String::new);
    let pass = use_state_eq(String::new);

    let label_opt = use_state_eq(|| StatusKind::Initial);

    let oninput_user = on_input(user.clone(), label_opt.clone());
    let oninput_pass = on_input(pass.clone(), label_opt.clone());
    let onsubmit = on_submit(user.clone(), pass.clone(), label_opt.clone());

    html! {
        <div style="display: flex; width: 100vw; height: 100vh; justify-content: center; align-items: center; text-align: center;">
            <div style="width: 45vw; height: 35vh; background-color: #142009; padding: 1vw; border-radius: 6px; border: 2px solid #5a5a5a;">
                <h2 style="color: #71b039">{ "Authenticated Minecraft Server Whitelister" }</h2>
                <form {onsubmit} style="width: 100%; height: 100%; display: flex; flex-direction: column; align-items: center;">
                    <input oninput={oninput_user} type="text" placeholder="Username" style="margin-top: 2vh; width: 48%; height: 16%; background-color: #181a1b; color: white; border-radius: 4px; border-color: #736b5e; font-size: large; text-align: center;"/>
                    <input oninput={oninput_pass} type="text" placeholder="Password" style="margin-top: 0.2vh; width: 48%; height: 16%; background-color: #181a1b; color: white; border-radius: 4px; border-color: #736b5e; font-size: large; text-align: center;"/>
                    <input type="submit" value="SUBMIT" style="width: 25%; height: 10%; background-color: #181a1b; color: white; border-radius: 6px; border-color: #736b5e; margin-top: 1vh;"/>

                    if label_opt.is_new() {
                        { label_opt.as_html() }
                    }
                </form>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
