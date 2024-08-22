mod pipeline;

use std::{collections::HashMap, thread};

use crate::traits::Runner;
pub use pipeline::Pipeline;

#[derive(Debug)]
pub struct Orchestrator {
    runners: Vec<Box<dyn Runner>>,
}

impl Orchestrator {
    pub fn new<const T: usize>(pipelines: [Pipeline; T]) -> Self {
        let mut node_mapping = HashMap::new();
        let mut runners = HashMap::new();

        for pipeline in pipelines {
            pipeline.evaluate(&mut node_mapping, &mut runners, None);
        }

        Self {
            runners: runners.into_values().collect(),
        }
    }

    pub fn run(self) {
        let mut handles = vec![];

        for runner in self.runners {
            let handle = thread::spawn(move || {
                runner.run();
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
