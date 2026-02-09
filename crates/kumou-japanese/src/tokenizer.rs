use lindera::dictionary::{load_embedded_dictionary, DictionaryKind};
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer;
use serde::{Deserialize, Serialize};

use crate::error::AnalysisError;

/// Part-of-speech information from IPADIC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PartOfSpeech {
    /// Major POS category (品詞): 名詞, 動詞, 助詞, etc.
    pub major: String,
    /// POS subcategory 1 (品詞細分類1)
    pub sub1: String,
    /// POS subcategory 2 (品詞細分類2)
    pub sub2: String,
    /// POS subcategory 3 (品詞細分類3)
    pub sub3: String,
}

/// A single analyzed token with grammar details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalyzedToken {
    /// Surface form as it appears in text
    pub surface: String,
    /// Part-of-speech information
    pub pos: PartOfSpeech,
    /// Conjugation type (活用型), e.g. 一段, 五段
    pub conjugation_type: String,
    /// Conjugation form (活用形), e.g. 基本形, 連用形
    pub conjugation_form: String,
    /// Base/dictionary form (原形)
    pub base_form: String,
    /// Katakana reading (読み)
    pub reading: String,
    /// Pronunciation (発音)
    pub pronunciation: String,
}

/// Result of analyzing a sentence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalyzedSentence {
    /// Original sentence text
    pub text: String,
    /// Analyzed tokens
    pub tokens: Vec<AnalyzedToken>,
}

/// Create a lindera tokenizer with IPADIC dictionary
pub fn create_tokenizer() -> Result<Tokenizer, AnalysisError> {
    let dictionary = load_embedded_dictionary(DictionaryKind::IPADIC)
        .map_err(|e| AnalysisError::TokenizerInit(e.to_string()))?;
    let segmenter = Segmenter::new(Mode::Normal, dictionary, None);
    Ok(Tokenizer::new(segmenter))
}

/// Analyze a Japanese sentence into tokens with grammar details
pub fn analyze_sentence(
    tokenizer: &Tokenizer,
    text: &str,
) -> Result<AnalyzedSentence, AnalysisError> {
    let mut tokens_result = tokenizer
        .tokenize(text)
        .map_err(|e| AnalysisError::Tokenization(e.to_string()))?;

    let mut analyzed_tokens = Vec::new();

    for token in tokens_result.iter_mut() {
        let details: Vec<String> = token
            .details()
            .iter()
            .map(|s| s.to_string())
            .collect();

        // IPADIC returns 9 fields:
        // [0] POS, [1] sub1, [2] sub2, [3] sub3,
        // [4] conjugation_type, [5] conjugation_form,
        // [6] base_form, [7] reading, [8] pronunciation
        let get = |i: usize| -> String {
            details
                .get(i)
                .cloned()
                .unwrap_or_else(|| "*".to_string())
        };

        analyzed_tokens.push(AnalyzedToken {
            surface: token.surface.to_string(),
            pos: PartOfSpeech {
                major: get(0),
                sub1: get(1),
                sub2: get(2),
                sub3: get(3),
            },
            conjugation_type: get(4),
            conjugation_form: get(5),
            base_form: get(6),
            reading: get(7),
            pronunciation: get(8),
        });
    }

    Ok(AnalyzedSentence {
        text: text.to_string(),
        tokens: analyzed_tokens,
    })
}

/// Get a CSS class name for a POS major category (for UI coloring)
pub fn pos_css_class(major: &str) -> &str {
    match major {
        "名詞" => "pos-noun",
        "動詞" => "pos-verb",
        "形容詞" => "pos-adjective",
        "副詞" => "pos-adverb",
        "助詞" => "pos-particle",
        "助動詞" => "pos-aux-verb",
        "接続詞" => "pos-conjunction",
        "感動詞" => "pos-interjection",
        "連体詞" => "pos-adnominal",
        "記号" => "pos-symbol",
        _ => "pos-other",
    }
}

/// Get English translation for a POS major category
pub fn pos_english(major: &str) -> &str {
    match major {
        "名詞" => "Noun",
        "動詞" => "Verb",
        "形容詞" => "i-Adjective",
        "形容動詞" => "na-Adjective",
        "副詞" => "Adverb",
        "助詞" => "Particle",
        "助動詞" => "Aux. Verb",
        "接続詞" => "Conjunction",
        "感動詞" => "Interjection",
        "連体詞" => "Adnominal",
        "記号" => "Symbol",
        "フィラー" => "Filler",
        _ => "Other",
    }
}
