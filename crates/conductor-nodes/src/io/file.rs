use conductor_core::{
    ports::{NodeConfigInputPort, NodeConfigOutputPort, NodeRunnerInputPort, NodeRunnerOutputPort},
    NodeConfig, NodeRunner,
};
use std::{
    fmt::Display,
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
};

struct FileReaderRunner<T: From<String> + Clone> {
    output: NodeRunnerOutputPort<T>,
    path: PathBuf,
}

impl<T: From<String> + Clone> NodeRunner for FileReaderRunner<T> {
    fn run(self: Box<Self>) {
        let mut file = OpenOptions::new().read(true).open(self.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let lines = contents.lines();

        for line in lines {
            self.output.send(&line.to_string().into());
        }
    }
}

pub struct FileReader<T: From<String> + Clone> {
    pub output: NodeConfigOutputPort<T>,
    path: PathBuf,
}

impl<T: From<String> + Clone> FileReader<T> {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            output: NodeConfigOutputPort::<T>::new(),
            path: path.as_ref().to_path_buf(),
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: From<String> + Clone + Send + 'static> NodeConfig for FileReader<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(FileReaderRunner {
            output: self.output.into(),
            path: self.path,
        })
    }
}

struct FileWriterRunner<T: Display> {
    input: NodeRunnerInputPort<T>,
    path: PathBuf,
}

impl<T: Display> NodeRunner for FileWriterRunner<T> {
    fn run(self: Box<Self>) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path)
            .unwrap();

        loop {
            writeln!(file, "{}", self.input.recv()).unwrap();
        }
    }
}

pub struct FileWriter<T: Display> {
    pub input: NodeConfigInputPort<T>,
    path: PathBuf,
}

impl<T: Display> FileWriter<T> {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            input: NodeConfigInputPort::<T>::new(),
            path: path.as_ref().to_path_buf(),
        }
    }
}

// TODO: Can + Send + 'static be removed?
impl<T: Display + Send + 'static> NodeConfig for FileWriter<T> {
    fn into_runner(self: Box<Self>) -> Box<dyn NodeRunner + Send> {
        Box::new(FileWriterRunner {
            input: self.input.into(),
            path: self.path,
        })
    }
}
