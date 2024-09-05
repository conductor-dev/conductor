pub use crate::pipeline;
use crate::{NodeConfig, NodeRunner};
use std::thread;

struct PipelineRunner {
    runners: Vec<Box<dyn NodeRunner + Send>>,
}

impl NodeRunner for PipelineRunner {
    fn run(self: Box<Self>) {
        let mut handles = vec![];

        for runner in self.runners {
            let handle = thread::spawn(move || runner.run());

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

pub struct Pipeline<I, O> {
    nodes: Vec<Box<dyn NodeConfig>>,
    pub input: I,
    pub output: O,
}

impl<I, O> Pipeline<I, O> {
    pub fn new(nodes: Vec<Box<dyn NodeConfig>>, input: I, output: O) -> Self {
        Self {
            nodes,
            input,
            output,
        }
    }
}

impl Pipeline<(), ()> {
    pub fn run(self) {
        Box::new(self).into_runner().run();
    }
}

impl<I, O> NodeConfig for Pipeline<I, O> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        // Make sure to drop the input and output ports here so the e.g. SourcePortCell to SourcePort conversion works
        drop(self.input);
        drop(self.output);

        Box::new(PipelineRunner {
            runners: self
                .nodes
                .into_iter()
                .map(|node| node.into_runner())
                .collect(),
        })
    }
}

#[macro_export]
macro_rules! pipeline {
    (
        [$($node:expr),*],
        ($($input:expr),*),
        ($($output:expr),*)
    ) => {
        {
            let cloned_input = ($($input.clone()),*);
            let cloned_output = ($($output.clone()),*);

            $crate::pipeline::Pipeline::new(
                vec![$(Box::new($node)),*],
                cloned_input,
                cloned_output
            )
        }
    };

    (
        [$($node:expr),*]
    ) => {
        pipeline!([$($node),*], (), ())
    };

    (
        $($node:expr),*
    ) => {
        pipeline!([$($node),*], (), ())
    };
}
