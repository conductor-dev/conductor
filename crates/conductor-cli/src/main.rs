use std::{
    mem::size_of,
    net::UdpSocket,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

fn main() -> () {
    let mut udp_receiver = UdpReceiverNode::new();
    let console_printer = ConsolePrinterNode::new();

    udp_receiver.output.connect(&console_printer.input);

    thread::spawn(move || udp_receiver.run());
    thread::spawn(move || console_printer.run());

    loop {}

    // TODO: join threads ?
}

/// SinkPort / InputPort
struct SinkPort<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> SinkPort<T> {
    fn new() -> Self {
        let (tx, rx) = channel::<T>();
        Self { tx, rx }
    }
}

/// SourcePort / OutputPort
struct SourcePort<T> {
    tx: Vec<Sender<T>>,
}

impl<T: Clone> SourcePort<T> {
    fn new() -> Self {
        Self { tx: Vec::new() }
    }

    fn connect(&mut self, input: &SinkPort<T>) {
        self.tx.push(input.tx.clone());
    }

    fn send(&self, value: &T) {
        for tx in &self.tx {
            tx.send(value.clone()).unwrap();
        }
    }
}

struct UdpReceiverNode {
    socket: UdpSocket,
    output: SourcePort<f32>,
}

impl UdpReceiverNode {
    fn new() -> Self {
        let socket = UdpSocket::bind("127.0.0.1:8080").unwrap();

        Self {
            socket,
            output: SourcePort::<f32>::new(),
        }
    }

    fn run(&self) {
        loop {
            let mut buffer = [0; size_of::<f32>()];
            self.socket.recv_from(&mut buffer).unwrap();
            let value = f32::from_ne_bytes(buffer);
            self.output.send(&value);
        }
    }
}

struct ConsolePrinterNode {
    input: SinkPort<f32>,
}

impl ConsolePrinterNode {
    fn new() -> Self {
        Self {
            input: SinkPort::<f32>::new(),
        }
    }

    fn run(&self) {
        loop {
            println!("received: {}", self.input.rx.recv().unwrap());
        }
    }
}
