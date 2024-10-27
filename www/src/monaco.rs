use monaco::sys::{editor, languages};
use wasm_bindgen::prelude::*;

pub const VS_DARK_PLUS: &str = "vs-dark-plus";
pub const RUST_PLUS: &str = "rust-plus";

pub fn setup_vs_dark_plus_theme() {
    THEME_VS_DARK_PLUS.with(|theme| {
        editor::define_theme(VS_DARK_PLUS, theme).unwrap();
    });
}

pub fn setup_rust_plus_language() {
    LANGUAGE_EXTENSION.with(|ext| {
        languages::register(ext);
    });
    LANGUAGE_CONFIG.with(|config| {
        languages::set_language_configuration(RUST_PLUS, config);
    });
    LANGUAGE_GRAMMAR.with(|grammar| {
        languages::set_monarch_tokens_provider(RUST_PLUS, grammar);
    });
}

#[wasm_bindgen(module = "/src/monaco-config.js")]
extern "C" {
    #[wasm_bindgen(thread_local, js_name = "languageExtension")]
    static LANGUAGE_EXTENSION: languages::ILanguageExtensionPoint;

    #[wasm_bindgen(thread_local, js_name = "themeVsDarkPlus")]
    static THEME_VS_DARK_PLUS: editor::IStandaloneThemeData;

    #[wasm_bindgen(thread_local, js_name = "languageConfig")]
    static LANGUAGE_CONFIG: languages::LanguageConfiguration;

    #[wasm_bindgen(thread_local, js_name = "grammar")]
    static LANGUAGE_GRAMMAR: languages::IMonarchLanguage;
}
