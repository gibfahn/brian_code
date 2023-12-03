/// A bubble sort algorithm could be used to sort the list of numbers into ascending order.
/// Takes a list of numbers and sorts them in-place.
pub fn bubble_sort(list: &mut [u64]) {
    loop {
        let mut unsorted_numbers = 0;
        for x in 0..list.len() - 1 {
            if list[x] > list[x + 1] {
                list.swap(x, x + 1);
                unsorted_numbers += 1;
            }
        }
        if unsorted_numbers == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bubble_sort::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        {
            let mut list = [3, 5, 8, 1, 6, 4];
            let expected_list = [1, 3, 4, 5, 6, 8];
            bubble_sort(&mut list);
            assert_eq!(expected_list, dbg!(list));
        }

        {
            let mut list_2 = [3, 5, 8, 1, 6, 9, 4];
            let expected_list_2 = [1, 3, 4, 5, 6, 8, 9];
            bubble_sort(&mut list_2);
            assert_eq!(expected_list_2, list_2);
        }
    }
}

//
//
//
//
//
//
//
//
//
//
//
//
//
// /// A bubble sort algorithm could be used to sort the list of numbers into ascending order.
// /// Takes a list of numbers and sorts them in-place.
// pub fn bubble_sort(list: &mut [u64]) {
//     let mut done = false;
//     while !done {
//         done = true;
//         for i in 0..list.len() - 1 {
//             if list[i] > list[i + 1] {
//                 list.swap(i, i + 1);
//                 done = false;
//             }
//         }
//     }
// }
