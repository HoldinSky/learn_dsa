use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub mod linked_list;
pub mod queue;
pub mod stack;

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T: Copy> {
    pub val: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            prev: None
        }
    }
}

impl <T: Copy> From<Node<T>> for Option<Rc<RefCell<Node<T>>>> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}