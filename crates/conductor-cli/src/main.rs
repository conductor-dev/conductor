use conductor::prelude::*;
use std::fmt::Display;

fn main() {
    let udp_receiver = UdpReceiver::<MyPacket>::new("127.0.0.1:8080");
    let norm = Normer::new();
    let udp_sender = UdpSender::new("127.0.0.1:0", "127.0.0.1:9090");
    let printer = ConsolePrinter::new();

    udp_receiver.output.connect(&norm.input);
    norm.output.connect(&udp_sender.input);
    norm.output.connect(&printer.input);

    pipeline![udp_receiver, norm, udp_sender, printer].run();
}

#[derive(Clone, Copy, Norm)]
struct MyPacket(f32);

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
