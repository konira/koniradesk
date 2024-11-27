#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::pages::{
    login_page::LoginPage,
    home_page::HomePage
};
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    HomePage {}, 

    #[route("/login")]
    LoginPage {}
}