#![allow(dead_code)]
use super::linked_list::List;

pub struct Queue<T: Copy> {
    list: List<T>
}

impl <T: Copy> Queue<T> {
    pub fn new() -> Self {
        Self { list: List::new() }
    }

    pub fn enqueue(&mut self, val: T) {
        self.list.push_back(val);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn peek(&self) -> Option<T> {
        self.list.get(0)
    }

    pub fn length(&self) -> usize {
        self.list.length()
    }
}