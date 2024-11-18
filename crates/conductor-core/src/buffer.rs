use std::collections::{vec_deque::Iter, VecDeque};

#[derive(Clone, Debug)]
pub struct CircularBuffer<T> {
    size: usize,
    buffer: VecDeque<T>,
}

impl<T> From<CircularBuffer<T>> for Vec<T> {
    fn from(buffer: CircularBuffer<T>) -> Self {
        buffer.buffer.into()
    }
}

impl<T> From<CircularBuffer<T>> for VecDeque<T> {
    fn from(buffer: CircularBuffer<T>) -> Self {
        buffer.buffer
    }
}

impl<T> From<Vec<T>> for CircularBuffer<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            size: vec.len(),
            buffer: vec.into(),
        }
    }
}

impl<T> From<VecDeque<T>> for CircularBuffer<T> {
    fn from(vec: VecDeque<T>) -> Self {
        let size = vec.len();
        Self { size, buffer: vec }
    }
}

impl<T> IntoIterator for CircularBuffer<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<T> CircularBuffer<T> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            buffer: VecDeque::with_capacity(size),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() == self.size {
            self.buffer.pop_front();
        }

        self.buffer.push_back(item);
    }

    pub fn resize(&mut self, new_size: usize) {
        if new_size < self.buffer.len() {
            self.buffer.drain(0..self.buffer.len() - new_size);
        } else {
            self.buffer.reserve(new_size - self.buffer.len());
        }

        self.size = new_size;
    }

    pub fn iter(&self) -> Iter<T> {
        self.buffer.iter()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_filled(&self) -> bool {
        self.len() == self.size()
    }
}
