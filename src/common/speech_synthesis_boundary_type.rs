/// SpeechSynthesisBoundaryType defines the boundary type of speech synthesis boundary event.
#[derive(Debug)]
pub enum SpeechSynthesisBoundaryType {
    /// WordBoundary indicates word boundary.
    WordBoundary = 0,

    /// PunctuationBoundary indicates punctuation boundary.
    PunctuationBoundary = 1,

    /// SentenceBoundary indicates sentence boundary.
    SentenceBoundary = 2,
}

impl SpeechSynthesisBoundaryType {
    #[cfg(not(target_os = "windows"))]
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => SpeechSynthesisBoundaryType::WordBoundary,
            1 => SpeechSynthesisBoundaryType::PunctuationBoundary,
            _ => SpeechSynthesisBoundaryType::SentenceBoundary,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn from_i32(value: i32) -> Self {
        SpeechSynthesisBoundaryType::from_u32(value as u32)
    }
}
