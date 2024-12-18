/// ProfanityOption defines the profanity option.
#[derive(Debug)]
pub enum ProfanityOption {
    /// Masked profanity option.
    Masked = 0,

    /// Removed profanity option
    Removed = 1,

    /// Raw profanity option
    Raw = 2,
}

impl From<ProfanityOption> for i32 {
    fn from(profanity_option: ProfanityOption) -> i32 {
        profanity_option as i32
    }
}

impl From<ProfanityOption> for u32 {
    fn from(profanity_option: ProfanityOption) -> u32 {
        profanity_option as u32
    }
}
