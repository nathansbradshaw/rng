use crate::components::RandomNumberGenerator;
use crate::components::NotFound;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    RandomNumberGenerator,
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
