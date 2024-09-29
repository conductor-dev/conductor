use conductor::prelude::*;

fn main() {
    let udp_receiver = UdpReceiver::<MyPacket>::new("127.0.0.1:8080");
    let udp_sender_1 = UdpSender::new("127.0.0.1:0", "127.0.0.1:9090");
    let udp_sender_2 = UdpSender::<MyPacket>::new("127.0.0.1:0", "127.0.0.1:9091");

    let fft = FFT::new(2000);
    let inverse_fft = InverseFFT::new(2000);

    udp_receiver.output.connect(&udp_sender_1.input);

    udp_receiver.output.connect(&fft.input);
    fft.output.connect(&inverse_fft.input);

    inverse_fft.output.connect(&udp_sender_2.input);

    pipeline![udp_receiver, udp_sender_1, udp_sender_2, fft, inverse_fft].run();
}

#[derive(Clone)]
struct MyPacket(f32);

impl From<MyPacket> for f32 {
    fn from(packet: MyPacket) -> Self {
        packet.0
    }
}

impl From<f32> for MyPacket {
    fn from(value: f32) -> Self {
        Self(value)
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
