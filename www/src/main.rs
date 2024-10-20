#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/playground")]
    Playground,
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Playground() -> Element {
    rsx! { "Hi" }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        "Page not found"
        Link {
            to: Route::Playground,
            "Go to playground"
        }
    }
}
