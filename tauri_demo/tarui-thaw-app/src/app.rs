use leptos::prelude::*;
use leptos::task::spawn_local;
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

    let http = move |name: &str| {
        let name = name.to_string();
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            let new_msg = invoke("http", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <Layout>
            <LayoutHeader attr:style="background-color: #0078ffaa; padding: 20px;">
                <Avatar shape=AvatarShape::Square size=36 />
                "卷闸门"
            </LayoutHeader>
            <Layout attr:style="background-color: #0078ff88; padding: 20px;">
                <Grid>
                    <GridItem>
                        <Image src="public/2c3b013418d55659.jpg" attr:style="width: 100%" shape=ImageShape::Rounded />
                    </GridItem>
                    <GridItem>
                        <p>{ move || greet_msg.get() }</p>
                    </GridItem>
                    <GridItem>
                        <Grid cols=2>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button attr:style="color: white;" on:click=move |_| {http("open");}>"开"</Button>
                                </Divider>
                            </GridItem>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button on:click=move |_| {http("close");} size=ButtonSize::Large>"关"</Button>
                                </Divider>
                            </GridItem>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button on:click=move |_| {http("stop");} size=ButtonSize::Large>"停"</Button>
                                </Divider>
                            </GridItem>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button on:click=move |_| {http("lock");} size=ButtonSize::Large>"锁"</Button>
                                </Divider>
                            </GridItem>
                        </Grid>
                    </GridItem>
                </Grid>
            </Layout>
        </Layout>
    }
}
