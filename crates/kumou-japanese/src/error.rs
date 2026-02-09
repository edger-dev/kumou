use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalysisError {
    #[error("Failed to initialize tokenizer: {0}")]
    TokenizerInit(String),

    #[error("Tokenization failed: {0}")]
    Tokenization(String),

    #[error("Failed to load dialogue data: {0}")]
    DataLoad(String),
}
