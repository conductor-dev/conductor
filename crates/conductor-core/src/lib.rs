pub mod nodes;
pub mod prelude;

use nodes::Node;
use std::thread;

pub fn run(nodes: Vec<Box<dyn Node + Send>>) {
    // pub fn run<const N: usize>(nodes: [Box<dyn Node + Send>; N]) {
    for node in nodes {
        thread::spawn(move || node.run());
    }

    loop {}
    // TODO: join threads?
}

// TODO
// #[macro_export]
// macro_rules! run {
//     ($($node:expr),*) => {
//         run_internal(vec![$(Box::new($node)),*]);
//     };
// }

// pub(crate) use run;
