mod app;
mod components;
mod pages;
mod router;

use app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
