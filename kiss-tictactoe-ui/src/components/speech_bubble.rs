use dioxus::prelude::*;

/// The engine's speech bubble (pure presentational).
///
/// The outer live region is stable so screen readers announce changes; the
/// inner bubble is re-keyed by `serial` so the CSS fade restarts on every new
/// message.
#[component]
pub fn SpeechBubble(message: Option<String>, serial: u32, is_final: bool) -> Element {
    let text = message.clone().unwrap_or_default();
    let empty = message.is_none();
    rsx! {
        div {
            class: "speech-bubble-region",
            role: "status",
            "aria-live": "polite",
            // Single-item keyed loop: a new serial gives a new key, so the
            // keyed diff replaces the element and the browser restarts the
            // CSS animation (D-11; no Rust timer).
            for key in std::iter::once(serial) {
                div {
                    key: "{key}",
                    class: "speech-bubble",
                    class: if is_final { "speech-bubble--final" } else { "speech-bubble--fading" },
                    class: if empty { "speech-bubble--empty" },
                    span { class: "speech-bubble-text", "{text}" }
                }
            }
        }
    }
}
