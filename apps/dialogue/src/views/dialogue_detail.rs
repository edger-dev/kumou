use crate::Route;
use crate::components::SentenceAnalysis;
use crate::server_fns::get_dialogue;
use dioxus::prelude::*;
use kumou_japanese::topic_name_ja;

const DIALOGUE_CSS: Asset = asset!("/assets/styling/dialogue.css");

#[component]
pub fn DialogueDetail(dialogue_id: u32) -> Element {
    let dialogue = use_server_future(move || get_dialogue(dialogue_id))?;
    let mut selected_sentence = use_signal(|| Option::<String>::None);

    rsx! {
        document::Link { rel: "stylesheet", href: DIALOGUE_CSS }

        div { class: "page-container",
            match &*dialogue.read() {
                Some(Ok(dialogue)) => rsx! {
                    Link {
                        to: Route::TopicDialogues { topic_id: dialogue.topic_id },
                        class: "back-link",
                        "← Back to {topic_name_ja(&dialogue.topic_name)}"
                    }

                    h1 { class: "page-title",
                        "Dialogue #{dialogue.dialogue_id}"
                    }
                    p { class: "page-subtitle",
                        "{topic_name_ja(&dialogue.topic_name)} ({dialogue.topic_name})"
                    }

                    div { class: "dialogue-conversation",
                        for utterance in &dialogue.utterances {
                            div {
                                class: "utterance utterance-{utterance.speaker}",
                                div { class: "utterance-bubble",
                                    div { class: "speaker-label speaker-{utterance.speaker}",
                                        "Speaker {utterance.speaker}"
                                    }
                                    p {
                                        class: "utterance-text",
                                        onclick: {
                                            let text = utterance.utterance.clone();
                                            move |_| {
                                                let current = selected_sentence();
                                                if current.as_deref() == Some(text.as_str()) {
                                                    selected_sentence.set(None);
                                                } else {
                                                    selected_sentence.set(Some(text.clone()));
                                                }
                                            }
                                        },
                                        "{utterance.utterance}"
                                    }
                                }
                            }
                        }
                    }

                    p { class: "hint-text", "Click any sentence to analyze its structure" }

                    if let Some(sentence) = selected_sentence() {
                        SentenceAnalysis { text: sentence }
                    }
                },
                Some(Err(e)) => rsx! { p { class: "error", "Error: {e}" } },
                None => rsx! { p { class: "loading", "Loading dialogue..." } },
            }
        }
    }
}
