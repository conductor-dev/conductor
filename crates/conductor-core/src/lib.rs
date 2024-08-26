pub mod nodes;
pub mod prelude;

use nodes::Node;
use std::thread;

pub fn run_internal(nodes: Vec<Box<dyn Node + Send>>) {
    let mut handles = vec![];

    for node in nodes {
        let handle = thread::spawn(move || node.run());

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
