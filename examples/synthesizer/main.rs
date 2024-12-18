mod audio_data_stream;
mod helpers;
mod speak_ssml_async;
mod speak_ssml_async_bm_viseme;
mod speak_text_async;
mod speak_text_async_2;
mod text_streaming;

#[tokio::main]
async fn main() {
    // requires MS Azure key for subscription with Cognitive Services enabled
    // for convenience MS subscription key can be put into file read by set_env_vars
    helpers::set_env_vars("/tmp/path/to/subscription/key");
    env_logger::init();

    speak_text_async::run_example().await;
    speak_text_async_2::run_example().await;
    speak_ssml_async::run_example().await;
    speak_ssml_async_bm_viseme::run_example().await;
    audio_data_stream::run_example().await;
    text_streaming::run_example().await;
}
