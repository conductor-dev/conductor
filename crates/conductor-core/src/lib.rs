pub mod node;
pub mod prelude;
pub mod runner;

use node::Node;
use std::thread;

pub fn run_internal(nodes: Vec<Box<dyn Node>>) {
    let mut handles = vec![];

    for node in nodes {
        let runner = node.create_runner();

        let handle = thread::spawn(move || runner.run());

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[macro_export]
macro_rules! run {
    ($($node:expr),*) => {
        conductor::core::run_internal(vec![$(Box::new($node)),*]);
    };
}
