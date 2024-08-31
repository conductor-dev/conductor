use std::collections::{vec_deque::Iter, VecDeque};

pub struct CircularBuffer<T> {
    size: usize,
    buffer: VecDeque<T>,
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
