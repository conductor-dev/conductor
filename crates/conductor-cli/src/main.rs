use conductor::prelude::*;
use std::fmt::Display;

fn main() {
    let sample_generator = SampleGenerator::<MyPacket>::new(31250);
    // let normer = Normer::new();

    let sin = Siner::new();

    let console_printer = ConsolePrinter::new();
    let udp_sender = UdpSender::new("127.0.0.1:0", "127.0.0.1:9090");

    sample_generator.output.connect(&sin.input);

    sin.output.connect(&console_printer.input);
    sin.output.connect(&udp_sender.input);

    pipeline![sample_generator, sin, console_printer, udp_sender].run();
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

impl Display for MyPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
