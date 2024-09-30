use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};
use std::thread;

struct AudioPlayerRunner {
    input: NodeRunnerInputPort<f32>,
    sample_rate: NodeRunnerOutputPort<usize>,
}

impl NodeRunner for AudioPlayerRunner {
    fn run(self: Box<Self>) {
        let host = cpal::default_host();
        let output_device = host.default_output_device().unwrap();
        let config: StreamConfig = output_device.default_output_config().unwrap().into();

        self.sample_rate.send(&(config.sample_rate.0 as usize));

        let stream = output_device
            .build_output_stream(
                &config,
                move |data, _: &_| {
                    // Play mono signal on every channel
                    let chunks = data.chunks_mut(config.channels as usize);

                    for chunk in chunks {
                        let input = self.input.recv().unwrap();

                        for sample in chunk {
                            *sample = input;
                        }
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
    pub sample_rate: NodeConfigOutputPort<usize>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self {
            input: NodeConfigInputPort::new(),
            sample_rate: NodeConfigOutputPort::new(),
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
            sample_rate: self.sample_rate.into(),
        })
    }
}
