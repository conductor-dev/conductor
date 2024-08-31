use conductor::prelude::*;
use std::{fmt::Display, ops::Add};

fn main() {
    let udp_receiver = UdpReceiver::<MyPacket>::new("127.0.0.1:8080");
    let fft = FFT::new(1024);
    let console_printer = ConsolePrinter::new();

    udp_receiver.output.connect(&fft.input);

    fft.output.connect(&console_printer.input);

    pipeline![udp_receiver, fft, console_printer].run();
}

#[derive(Clone, Copy, Norm)]
struct MyPacket(f32);

impl Add<f32> for MyPacket {
    type Output = MyPacket;

    fn add(self, rhs: f32) -> Self::Output {
        MyPacket(self.0 + rhs)
    }
}

impl From<MyPacket> for f64 {
    fn from(packet: MyPacket) -> Self {
        packet.0 as f64
    }
}

impl UdpDeserializer for MyPacket {
    fn max_packet_size() -> usize {
        size_of::<f32>()
    }

    fn deserialize_packet(bytes: &[u8]) -> Self {
        Self(f32::from_ne_bytes(bytes.try_into().unwrap()))
    }
}

impl Display for MyPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
