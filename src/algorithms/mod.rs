pub mod search;
pub mod sort;

#[cfg(test)]
mod test {
    use crate::algorithms::search::linear_search;
    use crate::algorithms::sort::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut array = [0, 52, -85, 24, 19, 8, 2, 100, -23];

        bubble_sort(&mut array);

        assert_eq!([-85, -23, 0, 2, 8, 19, 24, 52, 100], array);
    }

    #[test]
    fn test_linear_search() {
        let array = [0, 52, -85, 24, 19, 8, 2, 100, -23];

        assert_eq!(linear_search(&array, 24), Some(3));
        assert_eq!(linear_search(&array, 100), Some(7));
        assert_eq!(linear_search(&array, 8), Some(5));
        assert_eq!(linear_search(&array, -5), None);
    }

    #[test]
    fn test_binary_search() {
        let array = [-85, -23, 0, 2, 8, 19, 24, 52, 100];

        assert_eq!(linear_search(&array, 0), Some(2));
        assert_eq!(linear_search(&array, 100), Some(8));
        assert_eq!(linear_search(&array, -85), Some(0));
        assert_eq!(linear_search(&array, 324), None);
        assert_eq!(linear_search(&array, 93), None);
    }
}