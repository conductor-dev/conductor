use conductor::nodes::{
    print::ConsolePrinterNode,
    udp::{DeserializeFromBytes, UdpReceiverNode},
};
use std::fmt::Display;

// TODO: Group/Pipeline/Node ?
// TODO: PassNode ?
// TODO: cli.exe graph
// TODO: push to algi-..

fn main() -> () {
    let mut udp_receiver = UdpReceiverNode::<MyPacket>::new("127.0.0.1:8080");
    let console_printer = ConsolePrinterNode::new();

    udp_receiver.output.connect(&console_printer.input);

    conductor::run_internal(vec![Box::new(udp_receiver), Box::new(console_printer)]);
    // conductor::run![udp_receiver, console_printer];
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
