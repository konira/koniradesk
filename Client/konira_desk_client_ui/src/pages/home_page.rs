#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::remoteid_component::RemoteIdComponent;   

#[component]
pub fn HomePage() -> Element {   
    rsx! {
        link { rel: "stylesheet", href: "home_page.css" }
        main {
            div{
            class: "home_container ",
                span { class: "underline", "Konira Desk" }
                RemoteIdComponent{}
            }
        } 
    }
}