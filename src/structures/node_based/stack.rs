#![allow(dead_code)]
use super::linked_list::List;

pub struct Stack<T: Copy> {
    list: List<T>
}

impl <T: Copy> Stack<T> {
    pub fn new() -> Self {
        Self { list: List::new() }
    }

    pub fn offer(&mut self, val: T) {
        self.list.push_front(val);
    }

    pub fn poll(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn peek(&self) -> Option<T> {
        self.list.get(0)
    }

    pub fn size(&self) -> usize {
        self.list.length()
    }
}