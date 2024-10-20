use dioxus::prelude::*;

#[component]
pub fn DynamicLayout(left: Element, right: Element) -> Element {
    // The split between the two panes in terms of the left's
    // percentage of space.
    let mut split = use_signal(|| 50.0);
    let mut dragging = use_signal(|| false);
    let mut layout_element = use_signal(|| None);

    rsx! {
        div {
            class: "flex w-full h-full",
            onmounted: move |event| layout_element.set(Some(event.data())),
            onmousemove: move |event: Event<MouseData>| async move {
                if !*dragging.read() {
                    return;
                }

                let element = layout_element.unwrap().get_client_rect().await.unwrap();
                let cursor_x = event.data().coordinates().client().x;

                let offset = cursor_x - element.origin.x;
                let decimal = offset / element.size.width;

                split.set(decimal * 100.0);
            },
            div {
                style: format!("flex: {split} 1 0px;"),
                {left}
            }
            div {
                onmousedown: move |_| dragging.set(true),
                onmouseup: move |_| dragging.set(false),
                "[]"
            }
            div {
                style: format!("flex: {} 1 0px;", 100.0 - *split.read()),
                {right}
            }
        }
    }
}
