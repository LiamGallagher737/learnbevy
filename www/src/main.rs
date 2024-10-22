use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use dioxus::prelude::*;
use dioxus_logger::tracing;

use playground::Playground;

mod components;
mod play;
mod playground;

#[allow(dead_code)]
const SERVER_SOCKET: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);

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

    #[allow(clippy::unit_arg)]
    LaunchBuilder::new()
        .with_cfg(server_only! {
            dioxus::fullstack::Config::new().addr(SERVER_SOCKET)
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
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
