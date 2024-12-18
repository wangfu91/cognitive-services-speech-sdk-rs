use std::env;

use super::helpers;
use cognitive_services_speech_sdk_rs::{
    common::SpeechSynthesisOutputFormat,
    speech::{AudioDataStream, SpeechConfig, SpeechSynthesisRequest, SpeechSynthesizer},
};
use log::*;

#[allow(dead_code)]
pub async fn run_example() {
    info!("---------------------------------------------------");
    info!("running text_streaming example...");
    info!("---------------------------------------------------");

    let region = env::var("MSServiceRegion").unwrap();
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
    let input_stream = request.get_input_stream();
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

            // wait for a while to read the audio data
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let audio_stream = AudioDataStream::from_speech_synthesis_result(result).unwrap();
            let buffer = &mut [0u8; 32000];

            while audio_stream.can_read_data(32000) {
                let read_size = audio_stream.read(buffer).unwrap();
                info!("read_size: {}", read_size);
            }
        }
    }

    info!("example finished!");
}
