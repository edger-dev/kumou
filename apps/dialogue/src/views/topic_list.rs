use crate::Route;
use crate::server_fns::get_topics;
use dioxus::prelude::*;
use kumou_japanese::topic_name_ja;

const DIALOGUE_CSS: Asset = asset!("/assets/styling/dialogue.css");

#[component]
pub fn TopicList() -> Element {
    let topics = use_server_future(move || get_topics())?;

    rsx! {
        document::Link { rel: "stylesheet", href: DIALOGUE_CSS }

        div { class: "page-container",
            h1 { class: "page-title", "Japanese Daily Dialogues" }
            p { class: "page-subtitle", "Browse conversations by topic to practice Japanese" }

            div { class: "topic-grid",
                match &*topics.read() {
                    Some(Ok(topics)) => rsx! {
                        for topic in topics {
                            Link {
                                to: Route::TopicDialogues { topic_id: topic.topic_id },
                                class: "topic-card",
                                div { class: "topic-name-ja", "{topic_name_ja(&topic.topic_name)}" }
                                div { class: "topic-name-en", "{topic.topic_name}" }
                                div { class: "topic-count", "{topic.dialogue_count} dialogues" }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! { p { class: "error", "Error: {e}" } },
                    None => rsx! { p { class: "loading", "Loading topics..." } },
                }
            }
        }
    }
}
