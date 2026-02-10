use crate::components::SpeakButton;
use crate::server_fns::analyze_sentence;
use dioxus::prelude::*;
use kumou_japanese::{pos_css_class, pos_english};

/// Speaks the sentence and uses boundary events to track which token is being spoken.
/// Updates a global JS variable `__kumou_speaking_char` with the current charIndex,
/// and calls a Rust-side callback via a custom event when the boundary fires or speech ends.
fn speak_with_tracking(text: &str) {
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
            const voices = synth.getVoices();
            const jaVoice = voices.find(v => v.lang.startsWith('ja'));
            if (jaVoice) {{
                utter.voice = jaVoice;
            }}
            utter.onboundary = function(e) {{
                window.__kumou_speaking_char = e.charIndex;
                document.dispatchEvent(new CustomEvent('kumou-boundary', {{ detail: e.charIndex }}));
            }};
            utter.onend = function() {{
                window.__kumou_speaking_char = -1;
                document.dispatchEvent(new CustomEvent('kumou-boundary', {{ detail: -1 }}));
            }};
            utter.onerror = function() {{
                window.__kumou_speaking_char = -1;
                document.dispatchEvent(new CustomEvent('kumou-boundary', {{ detail: -1 }}));
            }};
            window.__kumou_speaking_char = 0;
            synth.speak(utter);
        }})();
        "#
    );

    document::eval(&js);
}

/// Given a charIndex from the boundary event, find which token index it falls into.
fn char_index_to_token(char_index: usize, ranges: &[(usize, usize, usize)]) -> Option<usize> {
    for &(start, end, token_idx) in ranges {
        if char_index >= start && char_index < end {
            return Some(token_idx);
        }
    }
    // If char_index is past all ranges, highlight the last token
    if let Some(&(start, _, last_idx)) = ranges.last() {
        if char_index >= start {
            return Some(last_idx);
        }
    }
    None
}

/// Build char-offset ranges using character counts (not byte counts) to match the Web Speech API.
fn build_char_offset_ranges(sentence: &str, surfaces: &[String]) -> Vec<(usize, usize, usize)> {
    let sentence_chars: Vec<char> = sentence.chars().collect();
    let mut ranges = Vec::new();
    let mut pos = 0usize; // character position
    for (i, surface) in surfaces.iter().enumerate() {
        let surf_chars: Vec<char> = surface.chars().collect();
        let surf_len = surf_chars.len();
        // Search for surface in remaining sentence characters
        let mut found = false;
        for start in pos..=sentence_chars.len().saturating_sub(surf_len) {
            if sentence_chars[start..start + surf_len] == surf_chars[..] {
                ranges.push((start, start + surf_len, i));
                pos = start + surf_len;
                found = true;
                break;
            }
        }
        if !found {
            // Fallback: assume contiguous
            ranges.push((pos, pos + surf_len, i));
            pos += surf_len;
        }
    }
    ranges
}

#[component]
pub fn SentenceAnalysis(text: String) -> Element {
    let analysis = use_server_future(move || {
        let text = text.clone();
        async move { analyze_sentence(text).await }
    });

    let mut speaking_token_idx = use_signal(|| None::<usize>);

    rsx! {
        div { class: "analysis-panel",
            h2 { class: "analysis-title", "Sentence Analysis" }

            match &*analysis.read() {
                Some(Ok(result)) => {
                    let surfaces: Vec<String> = result.tokens.iter().map(|t| t.surface.clone()).collect();
                    let sentence_text = result.text.clone();
                    let ranges = build_char_offset_ranges(&sentence_text, &surfaces);

                    // Set up a listener for the custom boundary events from JS
                    let ranges_for_eval = ranges.clone();
                    use_effect(move || {
                        let ranges = ranges_for_eval.clone();
                        let js = r#"
                            (function() {
                                // Clean up previous listener if any
                                if (window.__kumou_boundary_handler) {
                                    document.removeEventListener('kumou-boundary', window.__kumou_boundary_handler);
                                }
                                window.__kumou_boundary_handler = function(e) {
                                    window.__kumou_last_boundary = e.detail;
                                };
                                document.addEventListener('kumou-boundary', window.__kumou_boundary_handler);
                            })();
                        "#;
                        document::eval(js);

                        // Poll the boundary value periodically
                        spawn(async move {
                            loop {
                                let result = document::eval(
                                    r#"
                                    (function() {
                                        var v = window.__kumou_last_boundary;
                                        window.__kumou_last_boundary = undefined;
                                        return v !== undefined ? v : null;
                                    })()
                                    "#
                                ).await;

                                if let Ok(value) = result {
                                    if let Some(char_idx) = value.as_i64() {
                                        if char_idx < 0 {
                                            speaking_token_idx.set(None);
                                        } else {
                                            let token_idx = char_index_to_token(char_idx as usize, &ranges);
                                            speaking_token_idx.set(token_idx);
                                        }
                                    }
                                }

                                // Use JS setTimeout as a sleep since we're in the browser
                                let _ = document::eval("new Promise(r => setTimeout(r, 50))").await;
                            }
                        });
                    });

                    let text_for_speak = result.text.clone();
                    let current_idx = speaking_token_idx();

                    rsx! {
                        div { class: "analysis-original",
                            span { class: "label", "Original: " }
                            "{result.text}"
                            button {
                                class: if current_idx.is_some() { "speak-btn speaking-active" } else { "speak-btn" },
                                title: "Speak with highlight",
                                onclick: move |evt: Event<MouseData>| {
                                    evt.stop_propagation();
                                    speak_with_tracking(&text_for_speak);
                                },
                                "\u{1f50a}"
                            }
                        }

                        div { class: "token-flow",
                            for (idx, token) in result.tokens.iter().enumerate() {
                                div {
                                    class: {
                                        let base = format!("token-chip {}", pos_css_class(&token.pos.major));
                                        if current_idx == Some(idx) {
                                            format!("{base} speaking")
                                        } else {
                                            base
                                        }
                                    },
                                    div { class: "token-surface", "{token.surface}" }
                                    div { class: "token-reading", "{token.reading}" }
                                    div { class: "token-pos", "{pos_english(&token.pos.major)}" }
                                    SpeakButton { text: token.surface.clone(), small: true }
                                }
                            }
                        }

                        h3 { class: "detail-heading", "Token Details" }
                        div { class: "token-table-wrapper",
                            table { class: "token-table",
                                thead {
                                    tr {
                                        th { "Surface" }
                                        th { "Reading" }
                                        th { "Base Form" }
                                        th { "POS" }
                                        th { "POS Detail" }
                                        th { "Conjugation" }
                                        th { "" }
                                    }
                                }
                                tbody {
                                    for (idx, token) in result.tokens.iter().enumerate() {
                                        tr {
                                            class: {
                                                let base = pos_css_class(&token.pos.major).to_string();
                                                if current_idx == Some(idx) {
                                                    format!("{base} speaking")
                                                } else {
                                                    base
                                                }
                                            },
                                            td { class: "surface-cell", "{token.surface}" }
                                            td { "{token.reading}" }
                                            td { "{token.base_form}" }
                                            td {
                                                span { class: "pos-badge {pos_css_class(&token.pos.major)}",
                                                    "{pos_english(&token.pos.major)}"
                                                }
                                            }
                                            td { class: "pos-detail",
                                                "{token.pos.major}"
                                                if token.pos.sub1 != "*" {
                                                    " / {token.pos.sub1}"
                                                }
                                                if token.pos.sub2 != "*" {
                                                    " / {token.pos.sub2}"
                                                }
                                            }
                                            td {
                                                if token.conjugation_type != "*" {
                                                    span { class: "conj-type", "{token.conjugation_type}" }
                                                }
                                                if token.conjugation_form != "*" {
                                                    span { class: "conj-form", " ({token.conjugation_form})" }
                                                }
                                            }
                                            td {
                                                SpeakButton { text: token.surface.clone(), small: true }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! { p { class: "error", "Analysis error: {e}" } },
                None => rsx! { p { class: "loading", "Analyzing sentence..." } },
            }
        }
    }
}
