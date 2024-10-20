use dioxus::prelude::*;

use crate::components::{card::Card, dynamic_layout::DynamicLayout};

#[component]
pub fn Playground() -> Element {
    rsx! {
        div {
            class: "p-4 h-screen w-full",
            DynamicLayout {
                left: rsx!{
                    Card {
                        class: "p-4",
                        "Left"
                    },
                },
                right: rsx! {
                    Card {
                        class: "p-4",
                        "Right"
                    },
                },
            }
        }
    }
}
