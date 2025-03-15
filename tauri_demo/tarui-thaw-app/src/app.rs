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
struct TcpArgs {
    id: u32,
    cmd: u32,
}

#[component]
pub fn App() -> impl IntoView {
    let (greet_msg, set_greet_msg) = signal(String::new());

    let tcp = move |id: u32, cmd: u32| {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&TcpArgs { id, cmd }).unwrap();
            let new_msg = invoke("tcp", args).await.as_string().unwrap();
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
                                    <Button shape=ButtonShape::Circular attr:style="background-color: #ffffff; color: #000000;"  on:click=move |_| {tcp(0, 0);}>"开"</Button>
                                </Divider>
                            </GridItem>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button shape=ButtonShape::Circular attr:style="background-color: #ffffff; color: #000000;" on:click=move |_| {tcp(0, 1);}>"关"</Button>
                                </Divider>
                            </GridItem>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button shape=ButtonShape::Circular attr:style="background-color: #ffffff; color: #000000;" on:click=move |_| {tcp(0, 2);}>"停"</Button>
                                </Divider>
                            </GridItem>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button shape=ButtonShape::Circular attr:style="background-color: #ffffff; color: #000000;" on:click=move |_| {tcp(0, 3);}>"锁"</Button>
                                </Divider>
                            </GridItem>
                        </Grid>
                    </GridItem>
                    <GridItem>
                        <Grid cols=2>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button shape=ButtonShape::Circular attr:style="background-color: #ffffff; color: #000000;"  on:click=move |_| {tcp(1, 0);}>"开"</Button>
                                </Divider>
                            </GridItem>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button shape=ButtonShape::Circular attr:style="background-color: #ffffff; color: #000000;" on:click=move |_| {tcp(1, 1);}>"关"</Button>
                                </Divider>
                            </GridItem>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button shape=ButtonShape::Circular attr:style="background-color: #ffffff; color: #000000;" on:click=move |_| {tcp(1, 2);}>"停"</Button>
                                </Divider>
                            </GridItem>
                            <GridItem attr:style="padding: 20px;">
                                <Divider>
                                    <Button shape=ButtonShape::Circular attr:style="background-color: #ffffff; color: #000000;" on:click=move |_| {tcp(1, 3);}>"锁"</Button>
                                </Divider>
                            </GridItem>
                        </Grid>
                    </GridItem>
                </Grid>
            </Layout>
        </Layout>
    }
}
