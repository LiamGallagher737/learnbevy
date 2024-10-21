use dioxus::prelude::*;

use crate::components::{button::*, card::Card, dynamic_layout::DynamicLayout};

#[component]
pub fn Playground() -> Element {
    rsx! {
        div {
            class: "p-4 h-screen w-full",
            DynamicLayout {
                left: rsx!{
                    Editor {}
                },
                right: rsx! {
                    Card {
                        class: "p-4 h-full",
                        "Right"
                    },
                },
            }
        }
    }
}

/// This is everything on the left side of the divider
#[component]
fn Editor() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 h-full",
            Card {
                class: "p-4",
                "Toolbar"
            }
            div {
                class: "flex flex-row gap-4 h-full",
                Card {
                    class: "h-full",
                    Sidebar {}
                }
                Card {
                    class: "p-4 w-full h-full",
                    "Main"
                }
            }
        }
    }
}

#[component]
fn Sidebar() -> Element {
    rsx! {
        div {
            class: "flex flex-col",
            Button {
                onclick: |_| {},
                variant: BtnVariant::Ghost,
                class: "rounded-b-none",
                "E"
            }
            div {
                class: "bg-border shrink-0 h-px w-full",
            }
            Button {
                onclick: |_| {},
                variant: BtnVariant::Ghost,
                class: "rounded-none",
                "A"
            }
            div {
                class: "bg-border shrink-0 h-px w-full",
            }
            Button {
                onclick: |_| {},
                variant: BtnVariant::Ghost,
                class: "rounded-none",
                "C"
            }
            div {
                class: "bg-border shrink-0 h-px w-full",
            }
        }
    }
}
