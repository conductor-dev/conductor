use conductor::prelude::*;
use std::fmt::Display;

fn main() {
    // let sample_generator = SampleGenerator::<MyPacket>::new(31250);
    // let normer = Normer::new();

    let recorder = AudioRecorder::new();
    let player = AudioPlayer::new();
    let lambda = Lambdaer::new(|x: f32| MyPacket(x));
    let resampler = Resampler::new();

    let immediate = Immediate::new(48000);
    // immediate.output.connect(&resampler.input_sample_rate);

    recorder.sample_rate.connect(&resampler.input_sample_rate);
    player.sample_rate.connect(&resampler.output_sample_rate);

    // recorder.output.connect(&lambda.input);
    recorder.output.connect(&resampler.input);
    resampler.output.connect(&player.input);

    pipeline![recorder, player, lambda, resampler, immediate].run();
}

#[derive(Clone, Copy, Norm)]
struct MyPacket(f32);

impl Sample for MyPacket {
    fn sample(sample_rate: usize, current_sample: usize) -> Self {
        MyPacket(f32::sample(sample_rate, current_sample))
    }
}

impl Sin for MyPacket {
    type Output = Self;

    fn sin(self) -> Self {
        MyPacket(self.0.sin())
    }
}

impl UdpSerializer for MyPacket {
    fn serialize_packet(self) -> Vec<u8> {
        self.0.to_ne_bytes().to_vec()
    }
}

// impl UdpSerializer for MyPacket {
//     fn serialize_packet(self) -> Vec<u8> {
//         self.0.to_ne_bytes().to_vec()
//     }
// }

impl Display for MyPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
