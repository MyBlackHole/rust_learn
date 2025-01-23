use leptos::task::spawn_local;
use leptos::prelude::*;
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
    let (greet_msg, set_greet_msg) = signal(String::new());

    let http = move |_| {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: "black" }).unwrap();
            let new_msg = invoke("http", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <Card>
            <CardHeader>
                <Body1>
                    <b>"Header"</b>" 2025-01-23"
                </Body1>
                <CardHeaderDescription slot>
                    <Caption1>"卷闸门"</Caption1>
                </CardHeaderDescription>
            </CardHeader>
            <CardPreview>
                <img src="public/2c3b013418d55659.jpg" style="width: 100%"/>
            </CardPreview>
            <CardFooter>
            <p>{ move || greet_msg.get() }</p>
            </CardFooter>
            <ButtonGroup>
                <Button on:click=http size=ButtonSize::Large>"开"</Button>
                <Button on:click=http size=ButtonSize::Large>"关"</Button>
                <Button on:click=http size=ButtonSize::Large>"停"</Button>
                <Button on:click=http size=ButtonSize::Large>"锁"</Button>
            </ButtonGroup>
        </Card>
    }
}
