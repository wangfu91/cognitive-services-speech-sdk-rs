/// Defines the possible reasons a recognition result might not be recognized.
#[derive(Debug)]
pub enum NoMatchReason {
    /// <summary>
    /// Indicates that speech was detected, but not recognized.
    /// </summary>
    NotRecognized = 1,

    /// <summary>
    /// Indicates that the start of the audio stream contained only silence, and the service timed out waiting for speech.
    /// </summary>
    InitialSilenceTimeout = 2,

    /// <summary>
    /// Indicates that the start of the audio stream contained only noise, and the service timed out waiting for speech.
    /// </summary>
    InitialBabbleTimeout = 3,

    /// <summary>
    /// Indicates that the spotted keyword has been rejected by the keyword verification service.
    /// Added in version 1.5.0.
    /// </summary>
    KeywordNotRecognized = 4,

    /// <summary>
    /// Indicates that the audio stream contained only silence after the last recognized phrase.
    /// </summary>
    EndSilenceTimeout = 5,
}

impl NoMatchReason {
    pub fn from_u32(code: u32) -> Self {
        match code {
            1 => NoMatchReason::NotRecognized,
            2 => NoMatchReason::InitialSilenceTimeout,
            3 => NoMatchReason::InitialBabbleTimeout,
            4 => NoMatchReason::KeywordNotRecognized,
            _ => NoMatchReason::EndSilenceTimeout,
        }
    }

    pub fn from_i32(code: i32) -> Self {
        NoMatchReason::from_u32(code as u32)
    }
}
