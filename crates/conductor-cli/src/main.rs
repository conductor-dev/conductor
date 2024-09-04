use conductor::prelude::*;
use std::{fmt::Display, ops::Add};

fn main() {
    let udp_receiver = UdpReceiver::<MyPacket>::new("127.0.0.1:8080");
    let normer = Normer::new();

    let immediate = Immediate::new(MyPacket(10.0));
    let adder = Adder::new();

    let console_printer = ConsolePrinter::new();

    udp_receiver.output.connect(&normer.input);

    normer.output.connect(&adder.input1);
    immediate.output.connect(&adder.input2);

    normer.output.connect(&console_printer.input);
    adder.output.connect(&console_printer.input);

    pipeline![udp_receiver, normer, immediate, adder, console_printer].run();
}

#[derive(Clone, Copy, Norm)]
struct MyPacket(f32);

impl From<MyPacket> for f64 {
    fn from(packet: MyPacket) -> Self {
        packet.0 as f64
    }
}

impl Add for MyPacket {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
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
