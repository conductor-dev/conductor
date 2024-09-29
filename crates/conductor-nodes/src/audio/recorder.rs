use conductor_core::{
    ports::{NodeConfigOutputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread;

struct AudioRecorderRunner {
    output: NodeRunnerOutputPort<f32>,
}

impl NodeRunner for AudioRecorderRunner {
    fn run(self: Box<Self>) {
        let host = cpal::default_host();
        let input_device = host.default_input_device().unwrap();
        let config = input_device.default_input_config().unwrap().into();

        let stream = input_device
            .build_input_stream(
                &config,
                move |data, _: &_| {
                    // Convert input to mono signal
                    let chunks = data.chunks(config.channels as usize);

                    for chunk in chunks {
                        self.output.send(chunk.first().unwrap());
                    }
                },
                move |err| {
                    panic!("an error occurred on the input stream: {}", err);
                },
                None,
            )
            .unwrap();

        stream.play().unwrap();

        loop {
            thread::park();
        }
    }
}

pub struct AudioRecorder {
    pub output: NodeConfigOutputPort<f32>,
}

impl AudioRecorder {
    pub fn new() -> Self {
        Self {
            output: NodeConfigOutputPort::new(),
        }
    }
}

impl Default for AudioRecorder {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl NodeConfig for AudioRecorder {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(AudioRecorderRunner {
            output: self.output.into(),
        })
    }
}
