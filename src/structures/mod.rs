pub mod node_based;

#[cfg(test)]
mod tests {
    use crate::structures::node_based::queue::Queue;
    use crate::structures::node_based::stack::Stack;
    use crate::structures::node_based::linked_list::List;

    #[test]
    fn list_test() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.insert_at(4, 1);
        list.insert_at(5, 2);

        assert_eq!([3, 4, 5, 2, 1], list.traverse().as_slice());

        list.remove_at(2);
        list.remove_at(1);
        list.remove_at(1);

        assert_eq!([3, 1], list.traverse().as_slice());

        list.remove_at(0);
        list.remove_at(0);

        let empty: [i32; 0] = [];
        assert_eq!(empty, list.traverse().as_slice());
    }

    #[test]
    fn queue_test() {
        let mut queue = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(3, queue.length());
        assert_eq!(1, queue.dequeue().unwrap());
        assert_eq!(2, queue.dequeue().unwrap());
        assert_eq!(3, queue.dequeue().unwrap());
    }

    #[test]
    fn stack_test() {
        let mut stack = Stack::new();

        stack.offer(1);
        stack.offer(2);
        stack.offer(3);
        stack.offer(4);

        assert_eq!(4, stack.size());
        assert_eq!(4, stack.poll().unwrap());
        assert_eq!(3, stack.peek().unwrap());
        assert_eq!(3, stack.poll().unwrap());
        assert_eq!(2, stack.poll().unwrap());
        assert_eq!(1, stack.poll().unwrap());
    }
}
