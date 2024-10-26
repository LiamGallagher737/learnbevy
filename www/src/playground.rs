use crate::{
    components::{button::*, card::Card, dynamic_layout::DynamicLayout},
    play::{play, InstanceModule},
};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use std::{ops::Deref, rc::Rc};

#[component]
pub fn Playground() -> Element {
    let instance_module = use_signal(|| None);
    let instance_canvas = use_signal::<Option<web_sys::Element>>(|| None);

    let mut game_card_element = use_signal::<Option<Rc<MountedData>>>(|| None);
    let mut game_size = use_signal(|| (0.0, 0.0));

    use_effect(move || {
        if let Some(element) = instance_canvas.read().deref() {
            let (width, height) = *game_size.read();
            let _ =
                element.set_attribute("style", &format!("width: {width}px; height: {height}px;"));
        }
    });

    rsx! {
        div {
            class: "p-4 h-screen w-full",
            DynamicLayout {
                onresized: move |_| {
                    spawn(async move {
                        if let Some(element) = game_card_element.read().deref() {
                            let rect = element.get_client_rect().await.unwrap();
                            game_size.set((rect.size.width, rect.size.height));
                        }
                    });
                },
                left: rsx!{
                    Editor {
                        instance_module,
                        instance_canvas,
                    }
                },
                right: rsx! {
                    div {
                        class: "flex flex-col gap-4 h-full",
                        Card {
                            class: "aspect-video",
                            div {
                                id: "game-card",
                                class: "relative overflow-hidden h-full w-full rounded-lg",
                                onmounted: move |event: MountedEvent| game_card_element.set(Some(event.data())),
                            },
                        },
                        Card {
                            class: "h-full",
                        },
                    },
                },
            }
        }
    }
}

/// This is everything on the left side of the divider
#[component]
fn Editor(
    instance_module: Signal<Option<InstanceModule>>,
    instance_canvas: Signal<Option<web_sys::Element>>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 h-full",
            Card {
                class: "p-4",
                Button {
                    onclick: move |_| {
                        spawn(async move {
                            tracing::info!("{:?}", instance_module.read().is_some());
                            if let Some(module) = instance_module.read().deref() {
                                module.exit();
                            }
                            if let Some(element) = instance_canvas.read().deref() {
                                element.remove();
                            }
                            instance_module.set(None);
                            instance_canvas.set(None);
                            let result = play(
                                CODE.to_string(),
                                shared::BevyVersion::V0_14,
                                shared::RustChannel::Nightly,
                            )
                            .await;
                            match result {
                                Ok(Ok(res)) => {
                                    instance_module.set(Some(res.module));
                                    instance_canvas.set(res.canvas);
                                    tracing::info!("{}", res.stderr)
                                }
                                Ok(Err(err)) => tracing::error!("Failed to play: {err:?}"),
                                Err(err) => tracing::error!("Failed to play: {err:?}"),
                            }
                        });
                    },
                    class: "font-semibold",
                    "Play"
                }
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

const CODE: &str = r#"
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, change_clear_color)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    info!("Here is some info");
    warn!("Here is a warning");
    error!("Here is an error");
}

fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>, mut state: Local<bool>) {
    if input.just_pressed(KeyCode::Space) {
        info!("Changing color");
        *state = !*state;
        if *state {
            clear_color.0 = Color::srgb(0.0, 1.0, 0.0);
        } else {
            clear_color.0 = Color::srgb(0.0, 0.0, 1.0);
        }
    }
}
"#;
