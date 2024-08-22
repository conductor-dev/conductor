use conductor::prelude::*;

fn main() {
    let socket = UdpSocket::new("127.0.0.1:8080".parse().unwrap());

    let output = PrintOutput::new();

    let pass = Pass::new();

    let pipeline_one = socket.send_to(&pass);

    let pipeline_two = pipeline_one.clone().send_to(&output);

    let pipeline_three = pipeline_one.send_to(&output);

    let orchestrator = Orchestrator::new([pipeline_two, pipeline_three]);

    orchestrator.run()
}
