use crate::monaco::{setup_rust_plus_language, setup_vs_dark_plus_theme, RUST_PLUS, VS_DARK_PLUS};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use monaco::{
    api::{CodeEditor, CodeEditorOptions},
    sys::editor::IEditorMinimapOptions,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[component]
pub fn CodeEditor(editor: Signal<Option<CodeEditorInstance>>) -> Element {
    use_hook(move || {
        info!("starting monaco");

        let document = web_sys::window().unwrap().document().unwrap();
        let card_element = document.get_element_by_id("editor-card").unwrap();

        // Monaco's parent element must be empty (no hydration elements)
        let element = card_element
            .append_child(&document.create_element("div").unwrap())
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        element.set_class_name("h-full");

        setup_vs_dark_plus_theme();
        setup_rust_plus_language();

        let options = CodeEditorOptions::default()
            .with_language(RUST_PLUS.to_owned())
            .with_theme(VS_DARK_PLUS.to_owned())
            .with_value(CODE.to_owned())
            .with_automatic_layout(true)
            .to_sys_options();

        let minimap = IEditorMinimapOptions::default();
        minimap.set_enabled(Some(false));
        options.set_minimap(Some(&minimap));

        let code_editor = CodeEditor::create(&element, Some(options));
        editor.set(Some(CodeEditorInstance { code_editor }));

        // Fixes dynamic layout not being able to go smaller
        let editor_element = element.first_element_child().unwrap();
        editor_element.set_class_name(&format!("!absolute {}", editor_element.class_name()));
    });

    rsx! {}
}

pub struct CodeEditorInstance {
    code_editor: CodeEditor,
}

impl CodeEditorInstance {
    pub fn get_value(&self) -> String {
        self.code_editor.get_model().unwrap().get_value()
    }

    pub fn set_value(&self, value: &str) {
        self.code_editor.get_model().unwrap().set_value(value)
    }
}

const CODE: &str = r#"use bevy::prelude::*;

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
