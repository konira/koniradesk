use dioxus::prelude::*;

#[component]
pub fn LoginComponent()-> Element {
    let mut user_valid = use_signal(|| false);
    let mut pass_valid = use_signal(|| false);
    rsx! {
        link { rel: "stylesheet", href: "login_component.css" }
        div{
            class: "login_container",
            span { "Login" }
            span { "user is valid: {user_valid}" }
           
            div {
                class: "login_form",
                input { 
                    r#type: "text", 
                    placeholder: "Username", 
                }
                input {   
                     r#type: "password",
                     placeholder: "Password", 
                    }
                button { 
                    r#type: "submit",
                    onclick : move |_event: Event<MouseData>| { 
                                user_valid.set(true);
                                pass_valid.set(true);
                            },
                    "Login"
                }
            }
          }
    }
}