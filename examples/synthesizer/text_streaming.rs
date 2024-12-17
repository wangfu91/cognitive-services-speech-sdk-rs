use super::helpers;
use log::*;

#[allow(dead_code)]
pub async fn run_example() {
    let (mut speech_synthesizer, pull_stream) = helpers::speech_synthesizer();

    helpers::set_callbacks(&mut speech_synthesizer);

    


}