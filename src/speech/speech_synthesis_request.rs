use crate::common::PropertyCollection;
use crate::error::{convert_err, Result};
use crate::ffi::{
    speech_synthesis_request_create, speech_synthesis_request_finish,
    speech_synthesis_request_get_property_bag, speech_synthesis_request_release,
    speech_synthesis_request_send_text_piece, SmartHandle, SPXPROPERTYBAGHANDLE, SPXREQUESTHANDLE,
};
use std::ffi::CString;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct TextInputStream<'a> {
    parent: &'a SpeechSynthesisRequest,
}

impl<'a> TextInputStream<'a> {
    fn new(parent: &'a SpeechSynthesisRequest) -> Self {
        Self { parent }
    }

    pub fn write<S: AsRef<str>>(&self, text: S) -> Result<()> {
        let text_ref = text.as_ref();
        self.parent.send_text_piece(text_ref)
    }

    pub fn close(&self) -> Result<()> {
        log::info!("Closing text input stream");
        self.parent.finish_input()
    }
}

pub struct SpeechSynthesisRequest {
    pub(crate) handle: SmartHandle<SPXREQUESTHANDLE>,
    properties: PropertyCollection,
}

unsafe impl Sync for SpeechSynthesisRequest {}

impl SpeechSynthesisRequest {
    pub fn new_text_streaming_request() -> Result<Self> {
        unsafe {
            let mut request_handle: MaybeUninit<SPXREQUESTHANDLE> = MaybeUninit::uninit();
            let ret = speech_synthesis_request_create(
                true,
                false,
                std::ptr::null(),
                0,
                request_handle.as_mut_ptr(),
            );
            convert_err(ret, "Failed to create speech synthesis request handle")?;

            let mut prop_bag_handle: MaybeUninit<SPXPROPERTYBAGHANDLE> = MaybeUninit::uninit();
            let ret = speech_synthesis_request_get_property_bag(
                request_handle.assume_init(),
                prop_bag_handle.as_mut_ptr(),
            );
            convert_err(ret, "Failed to get speech synthesis request property bag")?;

            let request = Self {
                handle: SmartHandle::create(
                    "SpeechSynthesisRequest",
                    request_handle.assume_init(),
                    speech_synthesis_request_release,
                ),
                properties: PropertyCollection::from_handle(prop_bag_handle.assume_init()),
            };

            Ok(request)
        }
    }

    pub fn get_text_input_stream(&self) -> TextInputStream {
        TextInputStream::new(self)
    }

    fn send_text_piece(&self, text: &str) -> Result<()> {
        let c_text = CString::new(text)?;
        let text_len = c_text.as_bytes().len();
        unsafe {
            let ret = speech_synthesis_request_send_text_piece(
                self.handle.inner(),
                c_text.as_ptr(),
                text_len as u32,
            );
            convert_err(ret, "Failed to send text piece")
        }
    }

    fn finish_input(&self) -> Result<()> {
        unsafe {
            let ret = speech_synthesis_request_finish(self.handle.inner());
            convert_err(ret, "Failed to finish input")
        }
    }
}

impl std::fmt::Debug for SpeechSynthesisRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpeechSynthesisRequest")
            .field("handle", &self.handle)
            .field("properties", &self.properties)
            .finish()
    }
}
