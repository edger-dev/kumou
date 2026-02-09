use crate::server_fns::analyze_sentence;
use dioxus::prelude::*;
use kumou_japanese::{pos_css_class, pos_english};

#[component]
pub fn SentenceAnalysis(text: String) -> Element {
    let analysis = use_server_future(move || {
        let text = text.clone();
        async move { analyze_sentence(text).await }
    })?;

    rsx! {
        div { class: "analysis-panel",
            h2 { class: "analysis-title", "Sentence Analysis" }

            match &*analysis.read() {
                Some(Ok(result)) => rsx! {
                    div { class: "analysis-original",
                        span { class: "label", "Original: " }
                        "{result.text}"
                    }

                    div { class: "token-flow",
                        for token in &result.tokens {
                            div {
                                class: "token-chip {pos_css_class(&token.pos.major)}",
                                div { class: "token-surface", "{token.surface}" }
                                div { class: "token-reading", "{token.reading}" }
                                div { class: "token-pos", "{pos_english(&token.pos.major)}" }
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
                                }
                            }
                            tbody {
                                for token in &result.tokens {
                                    tr { class: pos_css_class(&token.pos.major),
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
