use crate::components::RandomNumberGenerator;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    RandomNumberGenerator
}