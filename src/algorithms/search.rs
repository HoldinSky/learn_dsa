#![allow(unused_assignments, dead_code)]

pub fn linear_search<T: Eq>(hay_stack: &[T], needle: T) -> Option<usize> {
    for i in 0..hay_stack.len() {
        if hay_stack[i] == needle {
            return Some(i);
        }
    }

    None
}

pub fn binary_search<T: Eq + Ord>(hay_stack: &[T], needle: T) -> Option<usize> {
    let mut low: usize = 0;
    let mut high = hay_stack.len() - 1;
    let mut middle: usize = 0;

    while low < high {
        middle = low + (high - low) / 2;
        let val: &T = &hay_stack[middle];

        if *val == needle {
            return Some(middle);
        } else if *val < needle {
            low = middle + 1;
        } else {
            high = middle;
        }
    };

    None
}

