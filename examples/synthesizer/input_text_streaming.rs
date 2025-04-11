use std::env;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::helpers;
use cognitive_services_speech_sdk_rs::{
    common::{SpeechSynthesisOutputFormat, StreamStatus},
    speech::{AudioDataStream, SpeechConfig, SpeechSynthesisRequest, SpeechSynthesizer},
};
use log::*;
use rodio::{OutputStream, Sink, Source};

// Define a custom PCM source for rodio
struct PcmSource {
    data: Vec<i16>,
    position: usize,
    sample_rate: u32,
    channels: u16,
}

impl PcmSource {
    pub fn new(data: Vec<u8>, sample_rate: u32, channels: u16) -> Self {
        // Convert raw PCM bytes to i16 samples
        let mut samples = Vec::with_capacity(data.len() / 2);

        for chunk in data.chunks_exact(2) {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            samples.push(sample);
        }

        PcmSource {
            data: samples,
            position: 0,
            sample_rate,
            channels,
        }
    }
}

impl Source for PcmSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some((self.data.len() - self.position) / self.channels as usize)
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        let seconds = (self.data.len() / self.channels as usize) as f32 / self.sample_rate as f32;
        Some(Duration::from_secs_f32(seconds))
    }
}

impl Iterator for PcmSource {
    type Item = i16;

    fn next(&mut self) -> Option<i16> {
        if self.position < self.data.len() {
            let sample = self.data[self.position];
            self.position += 1;
            Some(sample)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running text_streaming example...");
    info!("---------------------------------------------------");

    // Initialize audio output system
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Failed to create audio output stream");
    let sink = Arc::new(Mutex::new(
        Sink::try_new(&stream_handle).expect("Failed to create audio sink"),
    ));

    // https://learn.microsoft.com/en-us/azure/ai-services/speech-service/how-to-lower-speech-synthesis-latency?pivots=programming-language-cpp#input-text-streaming

    let region = env::var("MSServiceRegion").unwrap();
    // To use the text stream API, you have to use the websocket V2 endpoint.
    let tts_endpoint =
        format!("wss://{region}.tts.speech.microsoft.com/cognitiveservices/websocket/v2");
    let subscription_key = env::var("MSSubscriptionKey").unwrap();

    let mut speech_config =
        SpeechConfig::from_endpoint_with_subscription(tts_endpoint, subscription_key.to_string())
            .unwrap();
    speech_config
        .set_get_speech_synthesis_language("en-US".to_string())
        .unwrap();
    speech_config
        .set_get_speech_synthesis_voice_name("en-US-AvaMultilingualNeural".to_string())
        .unwrap();
    speech_config
        .set_speech_synthesis_output_format(SpeechSynthesisOutputFormat::Raw24Khz16BitMonoPcm)
        .unwrap();

    let mut speech_synthesizer = SpeechSynthesizer::from_config(speech_config, None).unwrap();

    helpers::set_callbacks(&mut speech_synthesizer);

    let request = SpeechSynthesisRequest::new_text_streaming_request().unwrap();
    let input_stream = request.get_text_input_stream();
    match speech_synthesizer.start_speaking_async(&request).await {
        Err(err) => {
            error!("speak_text_async error {:?}", err);
        }
        Ok(result) => {
            info!("speak_text_async success: {:?}", result);

            input_stream.write("Hello, ").unwrap();

            input_stream.write("we also announced that developers can access Phi Silica API starting in January 2025.").unwrap();
            input_stream
                .write("Developers can bring language intelligence capabilities into their apps")
                .unwrap();
            input_stream
                .write("without needing to worry about model optimization")
                .unwrap();
            input_stream
                .write("or customization as Phi Silica is pre-tuned and ships inbox.")
                .unwrap();

            input_stream.close().unwrap();

            let audio_stream = AudioDataStream::from_speech_synthesis_result(result).unwrap();
            let buffer = &mut [0u8; 32000]; // Buffer to receive audio data

            loop {
                let status = audio_stream.get_status().unwrap();
                match status {
                    StreamStatus::StreamStatusNoData => {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                    StreamStatus::StreamStatusCanceled => {
                        info!("audio_stream status: Canceled");
                        break;
                    }
                    StreamStatus::StreamStatusAllData => {
                        info!("audio_stream status: AllData");
                        break;
                    }
                    _ => {
                        info!("audio_stream status: {:?}", status);
                        let read_size = audio_stream.read(buffer).unwrap();

                        if read_size > 0 {
                            // Create a buffer with only the data we actually read
                            let audio_data = buffer[..read_size as usize].to_vec();

                            // Skip empty chunks
                            if !audio_data.is_empty() {
                                // Create a PCM source from the raw audio data
                                // Raw24Khz16BitMonoPcm format corresponds to:
                                // - 24000 Hz sample rate
                                // - 16-bit samples
                                // - 1 channel (mono)
                                let source = PcmSource::new(audio_data, 24000, 1);

                                // Play the audio chunk
                                let sink_lock = sink.lock().unwrap();
                                sink_lock.append(source);
                                sink_lock.play();
                            }
                        }
                    }
                }
            }

            // Wait for all audio to finish playing before exiting
            {
                let sink_lock = sink.lock().unwrap();
                sink_lock.sleep_until_end();
            }
        }
    }

    info!("example finished!");
}
