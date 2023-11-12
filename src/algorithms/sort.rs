pub fn bubble_sort<T: Ord + Clone>(array: &mut [T]) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j] > array[j + 1] {
                let tmp = array[j].clone();
                array[j] = array[j + 1].clone();
                array[j + 1] = tmp;
            }
        }
    }
}