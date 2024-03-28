use super::helpers;
use cognitive_services_speech_sdk_rs::audio::AudioConfig;
use cognitive_services_speech_sdk_rs::speech::{EmbeddedSpeechConfig, SpeechRecognizer};
use log::*;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[allow(dead_code)]
pub async fn run_example() {
    info!("-----------------------------------------------");
    info!("running embedded recognize_from_file example...");
    info!("-----------------------------------------------");

    //let filename = helpers::get_sample_file("hello_rust.wav");

    let filename = "./output_recorded.wav";
    let audio_config = AudioConfig::from_wav_file_input(&filename).unwrap();

    let mut speech_config =
        EmbeddedSpeechConfig::from_path(env::var("embedded-speech-model-path").unwrap()).unwrap();

    let models = speech_config.get_speech_recognition_models().unwrap();
    let model = models.last().unwrap();
    info!("Using first model: {:?}", model);

    speech_config
        .set_speech_recognition_model(model, env::var("embedded-speech-model-key").unwrap())
        .unwrap();

    let mut speech_recognizer =
        SpeechRecognizer::from_embedded_config(speech_config, audio_config).unwrap();

    let result = speech_recognizer.recognize_once_async().await;

    info!("got recognition {:?}", result);
    info!("example finished!");
}
