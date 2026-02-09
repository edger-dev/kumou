use dioxus::prelude::*;

/// Speaks the given text using the browser's Web Speech API with Japanese voice.
/// Cancels any ongoing speech before starting.
fn speak_japanese(text: &str) {
    // Escape text for JavaScript string literal
    let escaped = text
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('\n', "\\n");

    let js = format!(
        r#"
        (function() {{
            const synth = window.speechSynthesis;
            synth.cancel();
            const utter = new SpeechSynthesisUtterance('{escaped}');
            utter.lang = 'ja-JP';
            utter.rate = 0.9;
            // Try to find a Japanese voice
            const voices = synth.getVoices();
            const jaVoice = voices.find(v => v.lang.startsWith('ja'));
            if (jaVoice) {{
                utter.voice = jaVoice;
            }}
            synth.speak(utter);
        }})();
        "#
    );

    document::eval(&js);
}

#[component]
pub fn SpeakButton(text: String, #[props(default = false)] small: bool) -> Element {
    let class = if small {
        "speak-btn speak-btn-small"
    } else {
        "speak-btn"
    };

    rsx! {
        button {
            class: class,
            title: "Speak",
            onclick: move |evt: Event<MouseData>| {
                evt.stop_propagation();
                speak_japanese(&text);
            },
            "\u{1f50a}"
        }
    }
}
