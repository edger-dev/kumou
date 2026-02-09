use dioxus::prelude::*;
use kumou_japanese::{AnalyzedSentence, Dialogue, DialoguePage, TopicSummary};

const TOPIC1_JSON: &str = include_str!("../assets/data/japanese-daily-dialogue/topic1.json");
const TOPIC2_JSON: &str = include_str!("../assets/data/japanese-daily-dialogue/topic2.json");
const TOPIC3_JSON: &str = include_str!("../assets/data/japanese-daily-dialogue/topic3.json");
const TOPIC4_JSON: &str = include_str!("../assets/data/japanese-daily-dialogue/topic4.json");
const TOPIC5_JSON: &str = include_str!("../assets/data/japanese-daily-dialogue/topic5.json");

fn load_all_dialogues() -> Result<Vec<Dialogue>, ServerFnError> {
    let mut all = Vec::new();
    for json in [TOPIC1_JSON, TOPIC2_JSON, TOPIC3_JSON, TOPIC4_JSON, TOPIC5_JSON] {
        let dialogues: Vec<Dialogue> =
            kumou_japanese::load_dialogues(json).map_err(|e| ServerFnError::new(e.to_string()))?;
        all.extend(dialogues);
    }
    Ok(all)
}

#[post("/api/topics")]
pub async fn get_topics() -> Result<Vec<TopicSummary>> {
    let dialogues = load_all_dialogues()?;

    let mut topics: Vec<TopicSummary> = Vec::new();
    for d in &dialogues {
        if let Some(t) = topics.iter_mut().find(|t| t.topic_id == d.topic_id) {
            t.dialogue_count += 1;
        } else {
            topics.push(TopicSummary {
                topic_id: d.topic_id,
                topic_name: d.topic_name.clone(),
                dialogue_count: 1,
            });
        }
    }
    topics.sort_by_key(|t| t.topic_id);
    Ok(topics)
}

#[post("/api/dialogues_by_topic")]
pub async fn get_dialogues_by_topic(
    topic_id: u32,
    page: usize,
    per_page: usize,
    search: String,
) -> Result<DialoguePage> {
    let dialogues = load_all_dialogues()?;

    let per_page = if per_page == 0 { 20 } else { per_page.min(100) };

    let filtered: Vec<Dialogue> = dialogues
        .into_iter()
        .filter(|d| d.topic_id == topic_id)
        .filter(|d| {
            if search.is_empty() {
                true
            } else {
                d.utterances
                    .iter()
                    .any(|u| u.utterance.contains(&search))
            }
        })
        .collect();

    let total = filtered.len();
    let total_pages = (total + per_page - 1) / per_page;
    let page = page.min(total_pages.saturating_sub(1));

    let start = page * per_page;
    let page_dialogues: Vec<Dialogue> = filtered
        .into_iter()
        .skip(start)
        .take(per_page)
        .collect();

    Ok(DialoguePage {
        dialogues: page_dialogues,
        total,
        page,
        per_page,
        total_pages,
    })
}

#[post("/api/dialogue")]
pub async fn get_dialogue(dialogue_id: u32) -> Result<Dialogue> {
    let dialogues = load_all_dialogues()?;

    Ok(dialogues
        .into_iter()
        .find(|d| d.dialogue_id == dialogue_id)
        .ok_or_else(|| ServerFnError::new("Dialogue not found"))?)
}

#[post("/api/analyze")]
pub async fn analyze_sentence(text: String) -> Result<AnalyzedSentence> {
    #[cfg(feature = "tokenizer")]
    {
        let tokenizer = kumou_japanese::create_tokenizer()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(kumou_japanese::analyze_sentence(&tokenizer, &text)
            .map_err(|e| ServerFnError::new(e.to_string()))?)
    }

    #[cfg(not(feature = "tokenizer"))]
    {
        let _ = text;
        Err(ServerFnError::new(
            "Tokenizer not available: build with 'tokenizer' feature to enable sentence analysis",
        ).into())
    }
}
