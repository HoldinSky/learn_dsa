#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::{Node, NodePtr};

pub struct List<T: Copy> {
    head: NodePtr<T>,
    tail: NodePtr<T>,
    length: usize,
}

impl<T: Copy> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, val: T) {
        self.length += 1;
        let mut node = Node::new(val);

        match &self.head.take() {
            Some(current_head) => {
                node.next = Some(Rc::clone(&current_head));
                self.head = node.into();

                if let Some(head) = &self.head {
                    current_head.borrow_mut().prev = Some(Rc::downgrade(head));
                }
            }
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
        };
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match &mut self.head.take() {
            None => None,
            Some(head) => {
                self.length -= 1;

                let mut head = head.borrow_mut();
                let next = head.next.take();
                match next {
                    None => {
                        self.tail.take();
                    }
                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next);
                    }
                }

                Some(head.val)
            }
        }
    }

    pub fn push_back(&mut self, val: T) {
        self.length += 1;
        let mut node = Node::new(val);

        match &self.tail.take() {
            Some(current_tail) => {
                node.prev = Some(Rc::downgrade(&current_tail));
                self.tail = node.into();

                current_tail.borrow_mut().next = self.tail.clone();
            }
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
        }
    }


    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.tail.take() {
            None => None,
            Some(tail) => {
                self.length -= 1;

                let mut tail = tail.borrow_mut();
                let prev = tail.prev.take();
                match prev {
                    None => {
                        self.head.take();
                    }
                    Some(prev) => {
                        let prev = prev.upgrade();
                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                }

                Some(tail.val)
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index == 0 {
            return Some(self.head.as_ref().unwrap().borrow().val);
        } else if index >= self.length - 1 {
            return Some(self.tail.as_ref().unwrap().borrow().val);
        }

        let node = self.get_to_nth_node(index);
        Some(node.unwrap().borrow().val)
    }

    pub fn insert_at(&mut self, val: T, index: usize) {
        if index == 0 {
            return self.push_front(val);
        } else if index >= self.length - 1 {
            return self.push_back(val);
        }

        let node = Rc::new(RefCell::new(Node::new(val)));
        let current = self.get_to_nth_node(index);

        node.borrow_mut().next = current.clone();
        node.borrow_mut().prev = current.clone().unwrap().borrow().prev.clone();
        node.borrow_mut().prev.as_ref().unwrap().upgrade().unwrap().borrow_mut().next = Some(node.clone());

        current.unwrap().borrow_mut().prev = Some(Rc::downgrade(&node));
        self.length += 1;
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            return self.pop_front();
        } else if index >= self.length - 1 {
            return self.pop_back();
        }

        let current = self.get_to_nth_node(index);

        let prev = current.as_ref().unwrap().borrow().prev.as_ref().unwrap().upgrade();
        prev.as_ref().unwrap().borrow_mut().next = current.as_ref().unwrap().borrow().next.clone();
        prev.as_ref().unwrap().borrow_mut().next.as_ref().unwrap().borrow_mut().prev =
            Some(Rc::downgrade(&prev.as_ref().unwrap()));

        self.length -= 1;

        let val = current.unwrap().borrow().val;
        Some(val)
    }

    pub fn traverse(&self) -> Vec<T> {
        let mut vals = vec![];
        let mut pointer = self.head.clone();

        while let Some(curr) = pointer.clone() {
            let curr = curr.borrow();
            vals.push(curr.val);
            pointer = curr.next.clone();
        }

        vals
    }

    fn get_to_nth_node(&self, index: usize) -> NodePtr<T> {
        let mut index = index;

        if index > self.length / 2 {
            // traverse from end
            let mut pointer = self.tail.clone();
            while index < self.length - 1 && match pointer {
                None => false,
                Some(_) => true
            } {
                pointer = Weak::upgrade(pointer.unwrap().borrow().prev.as_ref().unwrap());
                index += 1;
            }

            pointer
        } else {
            // traverse from start
            let mut pointer = self.head.clone();
            while index > 0 && match pointer {
                None => false,
                Some(_) => true
            } {
                pointer = pointer.unwrap().borrow().next.clone();
                index -= 1;
            }

            pointer
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

impl<T: Copy> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}