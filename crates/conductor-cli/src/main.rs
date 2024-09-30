use conductor::prelude::*;
use std::fmt::Display;

fn main() {
    // let sample_generator = SampleGenerator::<MyPacket>::new(31250);
    // let normer = Normer::new();

    let recorder = AudioRecorder::new();
    let player = AudioPlayer::new();

    let immediate = Immediate::new(5.0);
    let multiply = Multiplier::new();

    let downsampler = Downsampler::new(5000);
    let console_printer = ConsolePrinter::new();
    let lambda = Lambdaer::new(|x: f32| MyPacket(x));
    let udp_sender = UdpSender::new("127.0.0.1:0", "127.0.0.1:9090");

    let fft = FFT::new(1024);
    let inverse_fft = InverseFFT::new(1024);

    immediate.output.connect(&multiply.input1);
    recorder.output.connect(&multiply.input2);

    multiply.output.connect(&downsampler.input);
    downsampler.output.connect(&console_printer.input);

    multiply.output.connect(&fft.input);
    fft.output.connect(&inverse_fft.input);

    inverse_fft.output.connect(&lambda.input);
    inverse_fft.output.connect(&player.input);
    lambda.output.connect(&udp_sender.input);

    pipeline![
        downsampler,
        recorder,
        player,
        lambda,
        console_printer,
        udp_sender,
        fft,
        inverse_fft,
        multiply,
        immediate
    ]
    .run();
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
