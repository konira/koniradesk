#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::router::Route;
pub fn App() -> Element {
    rsx! {
      link { rel: "stylesheet", href: "styles.css" }
      Router::<Route> {} 
    }
}