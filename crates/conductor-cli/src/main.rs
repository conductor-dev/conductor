use conductor::prelude::*;

fn main() {
    let frequency = Immediate::new(2000.0);
    let amplitude = Immediate::new(5.0);

    let sample_generator = SampleGenerator::<f32>::new();
    let multiplier_1 = Multiplier::new();
    let sin = Siner::new();
    let multiplier_2 = Multiplier::new();
    let player = AudioPlayer::new();
    let into = Intoer::new();
    let udp_sender = UdpSender::<MyPacket>::new("127.0.0.1:0", "127.0.0.1:8080");

    sample_generator.output.connect(&multiplier_1.input1);
    frequency.output.connect(&multiplier_1.input2);
    multiplier_1.output.connect(&sin.input);
    sin.output.connect(&multiplier_2.input1);
    amplitude.output.connect(&multiplier_2.input2);
    // multiplier_2.output.connect(&player.input);
    player.sample_rate.connect(&sample_generator.sample_rate);
    multiplier_2.output.connect(&into.input);
    into.output.connect(&udp_sender.input);

    pipeline![
        sample_generator,
        frequency,
        multiplier_1,
        multiplier_2,
        amplitude,
        sin,
        player,
        into,
        udp_sender
    ]
    .run();
}

#[derive(Clone)]
struct MyPacket(f32);

impl From<f32> for MyPacket {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl UdpSerializer for MyPacket {
    fn serialize_packet(self) -> Vec<u8> {
        self.0.to_ne_bytes().to_vec()
    }
}
