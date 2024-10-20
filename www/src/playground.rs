use dioxus::prelude::*;

use crate::components::dynamic_layout::DynamicLayout;

#[component]
pub fn Playground() -> Element {
    rsx! {
        div {
            class: "p-4 h-screen w-full",
            DynamicLayout {
                left: rsx!{},
                right: rsx! {},
            }
        }
    }
}
