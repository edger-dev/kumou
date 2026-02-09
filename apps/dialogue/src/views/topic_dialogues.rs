use crate::Route;
use crate::server_fns::get_dialogues_by_topic;
use dioxus::prelude::*;
use kumou_japanese::topic_name_ja;

const DIALOGUE_CSS: Asset = asset!("/assets/styling/dialogue.css");

#[component]
pub fn TopicDialogues(topic_id: u32) -> Element {
    let dialogues = use_server_future(move || get_dialogues_by_topic(topic_id))?;

    rsx! {
        document::Link { rel: "stylesheet", href: DIALOGUE_CSS }

        div { class: "page-container",
            Link { to: Route::TopicList {}, class: "back-link", "← Back to Topics" }

            match &*dialogues.read() {
                Some(Ok(dialogues)) => {
                    let topic_name = dialogues.first().map(|d| d.topic_name.as_str()).unwrap_or("Unknown");
                    rsx! {
                        h1 { class: "page-title",
                            "{topic_name_ja(topic_name)} "
                            span { class: "title-en", "({topic_name})" }
                        }

                        div { class: "dialogue-list",
                            for dialogue in dialogues {
                                Link {
                                    to: Route::DialogueDetail { dialogue_id: dialogue.dialogue_id },
                                    class: "dialogue-card",
                                    div { class: "dialogue-header",
                                        span { class: "dialogue-id", "Dialogue #{dialogue.dialogue_id}" }
                                        span { class: "dialogue-turns", "{dialogue.dialogue_length} turns" }
                                    }
                                    div { class: "dialogue-preview",
                                        for utterance in dialogue.utterances.iter().take(2) {
                                            p { class: "preview-line",
                                                span { class: "speaker speaker-{utterance.speaker}", "{utterance.speaker}" }
                                                " {utterance.utterance}"
                                            }
                                        }
                                        if dialogue.utterances.len() > 2 {
                                            p { class: "preview-more", "..." }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! { p { class: "error", "Error: {e}" } },
                None => rsx! { p { class: "loading", "Loading dialogues..." } },
            }
        }
    }
}
