use nodes::Node;
use std::thread;

pub mod nodes;

pub fn run_internal(nodes: Vec<Box<dyn Node + Send>>) {
    for node in nodes {
        thread::spawn(move || node.run());
    }

    loop {}

    // TODO: join threads?
}

#[macro_export]
macro_rules! run {
    ($($node:expr),*) => {
        run_internal(vec![$(Box::new($node)),*]);
    };
}

// pub(crate) use run;
