use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use thaw::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let greet_reply = move |_| {
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <Card>
            <CardHeader>
                <Body1>
                    <b>"Header"</b>" 2022-02-22"
                </Body1>
                <CardHeaderDescription slot>
                    <Caption1>"Description"</Caption1>
                </CardHeaderDescription>
                <CardHeaderAction slot>
                    <Button appearance=ButtonAppearance::Transparent icon=icondata::AiMoreOutlined />
                </CardHeaderAction>
            </CardHeader>
            <CardPreview>
                <img src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" style="width: 100%"/>
            </CardPreview>
            <CardFooter>
                <Button on:click=greet_reply>"Reply"</Button>
                <Button>"Share"</Button>
            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>
            <p>{ move || greet_msg.get() }</p>
            </CardFooter>
        </Card>
    }
}
