#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::components::login_component::LoginComponent;   
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
pub fn LoginPage() -> Element {   
    rsx! {
        link { rel: "stylesheet", href: "login_page.css" }
        main {
            class: "login_page",
            LoginComponent{}
        }
    }
}