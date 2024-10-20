use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

const DEFAULT_CLASSES: &str = "bg-card text-card-foreground rounded-lg border shadow-sm";

#[component]
pub fn Card(class: Option<String>, children: Element) -> Element {
    let class = class
        .map(|c| tw_merge!(DEFAULT_CLASSES, c))
        .unwrap_or(DEFAULT_CLASSES.to_owned());

    rsx! {
        div {
            class,
            {children}
        }
    }
}
