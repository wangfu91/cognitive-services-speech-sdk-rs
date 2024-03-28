use super::helpers;
use cognitive_services_speech_sdk_rs::{
    audio::AudioConfig,
    speech::{EmbeddedSpeechConfig, SpeechRecognizer},
};
use log::*;
use std::{env, time::Duration};
use tokio::time::sleep;

#[allow(dead_code)]
pub async fn run_example() {
    info!("running embedded_from_speaker example...");

    let audio_config = AudioConfig::from_default_speaker_output().unwrap();

    let mut speech_config =
        EmbeddedSpeechConfig::from_path(env::var("embedded-speech-model-path").unwrap()).unwrap();

    let models = speech_config.get_speech_recognition_models().unwrap();
    let model = models.first().unwrap();
    info!("Using first model: {:?}", model);

    speech_config
        .set_speech_recognition_model(model, env::var("embedded-speech-model-key").unwrap())
        .unwrap();

    let mut speech_recognizer =
        SpeechRecognizer::from_embedded_config(speech_config, audio_config).unwrap();

    helpers::set_callbacks(&mut speech_recognizer);

    if let Err(err) = speech_recognizer.start_continuous_recognition_async().await {
        error!("start_continuous_recognition_async error {:?}", err);
    }
    sleep(Duration::from_millis(20000)).await;

    info!("example finished!");
}
