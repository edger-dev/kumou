use crate::Route;
use crate::server_fns::get_dialogues_by_topic;
use dioxus::prelude::*;
use kumou_japanese::topic_name_ja;

const DIALOGUE_CSS: Asset = asset!("/assets/styling/dialogue.css");

#[component]
pub fn TopicDialogues(topic_id: u32) -> Element {
    let mut current_page = use_signal(|| 0usize);
    let mut search_input = use_signal(|| String::new());
    let mut active_search = use_signal(|| String::new());
    let per_page = 20usize;

    let dialogues = use_server_future(move || {
        let search = active_search();
        let page = current_page();
        async move { get_dialogues_by_topic(topic_id, page, per_page, search).await }
    })?;

    rsx! {
        document::Link { rel: "stylesheet", href: DIALOGUE_CSS }

        div { class: "page-container",
            Link { to: Route::TopicList {}, class: "back-link", "← Back to Topics" }

            match &*dialogues.read() {
                Some(Ok(page_data)) => {
                    let topic_name = page_data.dialogues.first()
                        .map(|d| d.topic_name.as_str())
                        .unwrap_or("Unknown");
                    let total = page_data.total;
                    let total_pages = page_data.total_pages;
                    let current = page_data.page;
                    rsx! {
                        h1 { class: "page-title",
                            "{topic_name_ja(topic_name)} "
                            span { class: "title-en", "({topic_name})" }
                        }
                        p { class: "page-subtitle", "{total} dialogues" }

                        // Search bar
                        div { class: "search-bar",
                            input {
                                r#type: "text",
                                class: "search-input",
                                placeholder: "Search dialogues by Japanese text...",
                                value: "{search_input}",
                                oninput: move |e| {
                                    search_input.set(e.value());
                                },
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter {
                                        active_search.set(search_input());
                                        current_page.set(0);
                                    }
                                },
                            }
                            button {
                                class: "search-btn",
                                onclick: move |_| {
                                    active_search.set(search_input());
                                    current_page.set(0);
                                },
                                "Search"
                            }
                            if !active_search().is_empty() {
                                button {
                                    class: "search-clear-btn",
                                    onclick: move |_| {
                                        search_input.set(String::new());
                                        active_search.set(String::new());
                                        current_page.set(0);
                                    },
                                    "Clear"
                                }
                            }
                        }

                        // Dialogue list
                        div { class: "dialogue-list",
                            for dialogue in &page_data.dialogues {
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

                        if page_data.dialogues.is_empty() {
                            p { class: "no-results", "No dialogues found matching your search." }
                        }

                        // Pagination
                        if total_pages > 1 {
                            div { class: "pagination",
                                button {
                                    class: "page-btn",
                                    disabled: current == 0,
                                    onclick: move |_| {
                                        current_page.set(0);
                                    },
                                    "«"
                                }
                                button {
                                    class: "page-btn",
                                    disabled: current == 0,
                                    onclick: move |_| {
                                        current_page.set(current_page().saturating_sub(1));
                                    },
                                    "‹"
                                }

                                {render_page_numbers(current, total_pages, current_page)}

                                button {
                                    class: "page-btn",
                                    disabled: current + 1 >= total_pages,
                                    onclick: move |_| {
                                        current_page.set(current_page() + 1);
                                    },
                                    "›"
                                }
                                button {
                                    class: "page-btn",
                                    disabled: current + 1 >= total_pages,
                                    onclick: move |_| {
                                        current_page.set(total_pages - 1);
                                    },
                                    "»"
                                }
                            }
                            p { class: "page-info",
                                "Page {current + 1} of {total_pages}"
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

fn render_page_numbers(
    current: usize,
    total_pages: usize,
    mut current_page: Signal<usize>,
) -> Element {
    // Show up to 7 page buttons around current page
    let start = current.saturating_sub(3);
    let end = (start + 7).min(total_pages);
    let start = if end.saturating_sub(7) < start {
        end.saturating_sub(7)
    } else {
        start
    };

    rsx! {
        for p in start..end {
            button {
                class: if p == current { "page-btn page-btn-active" } else { "page-btn" },
                onclick: move |_| {
                    current_page.set(p);
                },
                "{p + 1}"
            }
        }
    }
}
