/// A bubble sort algorithm could be used to sort the list of numbers into ascending order.
/// Takes a list of numbers and sorts them in-place.
pub fn bubble_sort(list: &mut [u64]) {
    list.swap(1, 2);
}

#[cfg(test)]
mod tests {
    use crate::bubble_sort::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut list = [3, 5, 8, 1, 6, 4];
        let expected_list = [3, 5, 8, 1, 6, 4];
        bubble_sort(&mut list);
        assert_eq!(expected_list, list);
    }
}
