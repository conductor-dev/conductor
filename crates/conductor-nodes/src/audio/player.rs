use conductor_core::{
    ports::{NodeConfigInputPort, NodeRunnerInputPort},
    NodeConfig, NodeRunner,
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::thread;

struct AudioPlayerRunner {
    input: NodeRunnerInputPort<f32>,
}

impl NodeRunner for AudioPlayerRunner {
    fn run(self: Box<Self>) {
        let host = cpal::default_host();
        let output_device = host.default_output_device().unwrap();
        let config = output_device.default_output_config().unwrap();

        let stream = output_device
            .build_output_stream(
                &config.into(),
                move |data, _: &_| {
                    for sample in data {
                        *sample = self.input.recv().unwrap();
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

pub struct AudioPlayer {
    pub input: NodeConfigInputPort<f32>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::new(),
        }
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Can + Send + 'static be removed?
impl NodeConfig for AudioPlayer {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(AudioPlayerRunner {
            input: self.input.into(),
        })
    }
}
