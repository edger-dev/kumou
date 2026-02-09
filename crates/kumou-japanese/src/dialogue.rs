use serde::{Deserialize, Serialize};

/// A single utterance in a dialogue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Utterance {
    pub turn_num: u32,
    pub speaker: String,
    pub utterance: String,
}

/// A complete dialogue between speakers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dialogue {
    pub topic_id: u32,
    pub topic_name: String,
    pub dialogue_id: u32,
    pub dialogue_length: u32,
    pub utterances: Vec<Utterance>,
}

/// Summary info for a topic (without full dialogue data)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopicSummary {
    pub topic_id: u32,
    pub topic_name: String,
    pub dialogue_count: usize,
}

/// A paginated response of dialogues
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DialoguePage {
    pub dialogues: Vec<Dialogue>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

/// Load dialogues from a JSON string
pub fn load_dialogues(json: &str) -> Result<Vec<Dialogue>, serde_json::Error> {
    serde_json::from_str(json)
}

/// Get topic name in Japanese
pub fn topic_name_ja(topic_name: &str) -> &str {
    match topic_name {
        "Dailylife" => "日常生活",
        "School" => "学校",
        "Travel" => "旅行",
        "Health" => "健康",
        "Entertainment" => "エンターテインメント",
        _ => topic_name,
    }
}
