use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    components::{button::*, card::Card, dynamic_layout::DynamicLayout},
    play::play,
};

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
    let mut instance = use_signal(|| None);

    rsx! {
        div {
            class: "flex flex-col gap-4 h-full",
            Card {
                class: "p-4",
                button {
                    onclick: move |_| {
                        tracing::info!("12");
                        spawn(async move {
                            let result = play(
                                CODE.to_string(),
                                shared::BevyVersion::V0_14,
                                shared::RustChannel::Nightly,
                            )
                            .await;
                            match result {
                                Ok(Ok(res)) => {
                                    instance.set(Some(res.instance));
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
