use dioxus::prelude::*;

#[component]
pub fn RemoteIdComponent()-> Element {
    rsx! {
        link { rel: "stylesheet", href: "remoteid_component.css" }
        div {
            class: "remoteid_container",
                input {
                    r#type: "text",
                    placeholder: "ID",
                }
                button{
                    r#type: "submit",
                    onclick: move |_event: Event<MouseData>| {
                        
                    },
                    "Submit"
            }
        }
    }
}
