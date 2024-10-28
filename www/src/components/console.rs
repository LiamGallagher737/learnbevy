// Much of the code here can only run on the client and will crash the server if run
#![cfg_attr(not(feature = "web"), allow(dead_code))]

use derive_more::derive::{Display, FromStr};
use dioxus::prelude::*;
use js_sys::{Array, Function, Reflect};
use wasm_bindgen::{prelude::*, JsValue};

#[component]
pub fn Console(entries: Signal<Vec<ConsoleEntry>>) -> Element {
    #[cfg(feature = "web")]
    use_hook(move || {
        override_console_log(entries);
    });

    rsx! {
        for entry in &*entries.read() {
            match entry {
                ConsoleEntry::Output { text } => rsx! {
                    pre {
                        {text.clone()}
                    }
                },
                ConsoleEntry::Log { level, location, text } => rsx! {
                    div {
                        span {
                            {level.to_string()}
                        }
                        span {
                            {location.clone()}
                        }
                        {text.clone()}
                    }
                },
            }
        }
    }
}

#[derive(Debug)]
pub enum ConsoleEntry {
    Output {
        text: String,
    },
    Log {
        level: ConsoleLogLevel,
        location: String,
        text: String,
    },
}

#[derive(Debug, Display, FromStr, PartialEq, Eq)]
pub enum ConsoleLogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Replace `console.log` with a custom function that captures logs and sends them to the console
/// component. All logs are also sent to the original `console.log` under `log_old` to ensure they
/// still show up in the devtools console.
fn override_console_log(mut entries: Signal<Vec<ConsoleEntry>>) {
    let console = web_sys::window().unwrap().get("console").unwrap();
    let original_log = Reflect::get(&console, &"log".into()).expect("Failed to get console.log");

    Reflect::set(&console, &"log_old".into(), &original_log)
        .expect("Failed to set console.log as log_old");

    let log_handler = Closure::wrap(Box::new(move |arg: JsValue| {
        if let Some(entry) = arg.as_string().map(parse_log).flatten() {
            entries.push(entry);
        }

        let console = web_sys::window().unwrap().get("console").unwrap();
        let log_old = Reflect::get(&console, &"log_old".into()).unwrap();
        let log_old_fn: &Function = log_old.unchecked_ref();
        let args = if arg.is_array() {
            Array::from(&arg)
        } else {
            let arr = Array::new();
            arr.push(&arg);
            arr
        };
        let _ = log_old_fn.apply(&console, &args);
    }) as Box<dyn FnMut(JsValue)>);

    let console = web_sys::window().unwrap().get("console").unwrap();
    Reflect::set(
        &console,
        &"log".into(),
        log_handler.as_ref().unchecked_ref(),
    )
    .expect("Failed to override console.log");

    // Prevent the closure from being dropped
    log_handler.forget();
}

fn parse_log(message: String) -> Option<ConsoleEntry> {
    let mut parts = message
        .splitn(4, "%c")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim());
    let level = parts.next()?.parse::<ConsoleLogLevel>().ok()?;
    let location = parts.next()?.to_string();
    let text = parts.next()?.to_string();
    Some(ConsoleEntry::Log {
        level,
        location,
        text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log() {
        let value = String::from("%cINFO%c src/components/code_editor.rs:14%c starting monaco");
        let entry = parse_log(value).expect("Entry should parse");
        let ConsoleEntry::Log {
            level,
            location,
            text,
            ..
        } = entry
        else {
            panic!("Entry should be of variant `log`");
        };

        assert_eq!(level, ConsoleLogLevel::Info);
        assert_eq!(location, String::from("src/components/code_editor.rs:14"));
        assert_eq!(text, String::from("starting monaco"));
    }
}
