use std::ffi::CString;
use std::mem::MaybeUninit;

use crate::error::{convert_err, Result};
use crate::ffi::{
    grammar_phrase_handle_release, phrase_list_grammar_add_phrase, phrase_list_grammar_clear,
    phrase_list_grammar_from_recognizer_by_name, SmartHandle, SPXGRAMMARHANDLE,
};

use super::grammar_phrase::GrammarPhrase;
use super::SpeechRecognizer;

/// Represents a phrase list grammar for dynamic grammar scenarios.
/// Added in version 1.5.0.
#[derive(Debug)]
pub struct PhraseListGrammar {
    pub handle: SmartHandle<SPXGRAMMARHANDLE>,
}

impl PhraseListGrammar {
    /// Creates a phrase list grammar for the specified recognizer.
    pub fn from_recognizer(recognizer: &SpeechRecognizer) -> Result<PhraseListGrammar> {
        unsafe {
            let c_name = CString::new("")?;
            let mut handle: MaybeUninit<SPXGRAMMARHANDLE> = MaybeUninit::uninit();
            let ret = phrase_list_grammar_from_recognizer_by_name(
                handle.as_mut_ptr(),
                recognizer.handle.inner(),
                c_name.as_ptr(),
            );
            convert_err(ret, "PhraseListGrammar::from_recognizer error")?;
            Ok(PhraseListGrammar {
                handle: SmartHandle::create(
                    "PhraseListGrammar",
                    handle.assume_init(),
                    grammar_phrase_handle_release,
                ),
            })
        }
    }

    /// AddPhrase adds a simple phrase that may be spoken by the user.
    pub fn add_phrase(&self, text: &str) -> Result<()> {
        let grammar_phrase = GrammarPhrase::from_text(text.to_owned())?;
        let ret = unsafe {
            phrase_list_grammar_add_phrase(self.handle.inner(), grammar_phrase.handle.inner())
        };
        convert_err(ret, "PhraseListGrammar::add_phrase error")?;
        Ok(())
    }

    /// Clears all phrases from the phrase list grammar.
    pub fn clear(&self) -> Result<()> {
        let ret = unsafe { phrase_list_grammar_clear(self.handle.inner()) };
        convert_err(ret, "PhraseListGrammar::clear error")?;
        Ok(())
    }
}
