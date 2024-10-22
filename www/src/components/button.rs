use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "ring-offset-background focus-visible:ring-ring inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
)]
struct Btn {
    size: BtnSize,
    variant: BtnVariant,
}

#[allow(dead_code)]
#[derive(TwVariant, PartialEq)]
pub enum BtnSize {
    #[tw(default, class = "h-10 px-4 py-2")]
    Default,
    #[tw(class = "h-9 rounded-md px-3")]
    Small,
    #[tw(class = "h-11 rounded-md px-8")]
    Large,
    #[tw(class = "h-10 w-10")]
    Icon,
}

#[allow(dead_code)]
#[derive(TwVariant, PartialEq)]
pub enum BtnVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
    Destructive,
    #[tw(class = "border-input bg-background hover:bg-accent hover:text-accent-foreground border")]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[component]
pub fn Button(
    onclick: EventHandler,
    class: Option<String>,
    size: Option<BtnSize>,
    variant: Option<BtnVariant>,
    children: Element,
) -> Element {
    let styles = Btn {
        size: size.unwrap_or_default(),
        variant: variant.unwrap_or_default(),
    }
    .with_class(class.unwrap_or_default());

    rsx! {
        button {
            onclick: move |_| onclick.call(()),
            class: styles.as_class(),
            {children}
        }
    }
}
