use conductor::prelude::*;
use std::fmt::Display;

fn main() {
    let file_reader = FileReader::<MyPacket>::new("./output.csv");
    let lambda = Lambdaer::new(|x: MyPacket| x.0);
    let audio_player = AudioPlayer::new();

    file_reader.output.connect(&lambda.input);
    lambda.output.connect(&audio_player.input);

    pipeline![file_reader, audio_player, lambda].run();
}

#[derive(Clone)]
struct MyPacket(f32);

impl From<String> for MyPacket {
    fn from(s: String) -> Self {
        MyPacket(s.parse().unwrap())
    }
}

impl Sample for MyPacket {
    fn sample(sample_rate: usize, current_sample: usize) -> Self {
        MyPacket(f32::sample(sample_rate, current_sample))
    }
}
impl Display for MyPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
