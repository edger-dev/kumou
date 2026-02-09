use dioxus::prelude::*;
use kumou_japanese::{AnalyzedSentence, Dialogue, TopicSummary};

const DIALOGUES_JSON: &str = include_str!("../data/dialogues.json");

#[post("/api/topics")]
pub async fn get_topics() -> Result<Vec<TopicSummary>> {
    let dialogues: Vec<Dialogue> = kumou_japanese::load_dialogues(DIALOGUES_JSON)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

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
pub async fn get_dialogues_by_topic(topic_id: u32) -> Result<Vec<Dialogue>> {
    let dialogues: Vec<Dialogue> = kumou_japanese::load_dialogues(DIALOGUES_JSON)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let filtered: Vec<Dialogue> = dialogues
        .into_iter()
        .filter(|d| d.topic_id == topic_id)
        .collect();
    Ok(filtered)
}

#[post("/api/dialogue")]
pub async fn get_dialogue(dialogue_id: u32) -> Result<Dialogue> {
    let dialogues: Vec<Dialogue> = kumou_japanese::load_dialogues(DIALOGUES_JSON)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

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
