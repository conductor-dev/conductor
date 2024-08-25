use conductor::prelude::*;
use std::fmt::Display;

fn main() -> () {
    let mut udp_receiver = UdpReceiver::<MyPacket>::new("127.0.0.1:8080");
    let console_printer = ConsolePrinter::new();

    udp_receiver.output.connect(&console_printer.input);

    conductor::core::run(vec![Box::new(udp_receiver), Box::new(console_printer)]);
}

#[derive(Clone, Copy)]
struct MyPacket(f32);

impl DeserializeFromBytes for MyPacket {
    fn deserialize_from_bytes(bytes: &[u8]) -> Self {
        let mut buffer = [0; size_of::<f32>()];
        buffer.copy_from_slice(&bytes[..size_of::<f32>()]);
        let data = f32::from_ne_bytes(buffer);

        Self(data)
    }
}

impl Display for MyPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
