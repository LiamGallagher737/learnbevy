use dioxus::prelude::*;

#[component]
pub fn DynamicLayout(
    left: Element,
    right: Element,
    onresized: Option<EventHandler<()>>,
) -> Element {
    // The split between the two panes in terms of the left's
    // percentage of space.
    let mut split = use_signal(|| 50.0);
    let mut dragging = use_signal(|| false);
    let mut layout_element = use_signal(|| None);

    rsx! {
        div {
            class: "flex w-full h-full",
            onmounted: move |event| layout_element.set(Some(event.data())),
            onmouseup: move |_| dragging.set(false),
            onmousemove: move |event: Event<MouseData>| async move {
                if !*dragging.read() {
                    return;
                }

                let element = layout_element.unwrap().get_client_rect().await.unwrap();
                let cursor_x = event.data().coordinates().client().x;

                let offset = cursor_x - element.origin.x;
                let decimal = offset / element.size.width;
                let percent = (decimal * 100.0).clamp(20.0, 80.0);
                split.set(percent);

                if let Some(handler) = onresized {
                    handler.call(());
                }
            },
            div {
                style: format!("flex: {split} 1 0px;"),
                {left}
            }
            div {
                class: "px-4",
                onmousedown: move |_| dragging.set(true),
                div {
                    class: "bg-border w-px h-full"
                }
            }
            div {
                style: format!("flex: {} 1 0px;", 100.0 - *split.read()),
                {right}
            }
        }
    }
}
