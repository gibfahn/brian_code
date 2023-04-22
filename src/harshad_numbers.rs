/**
<https://filestore.aqa.org.uk/sample-papers-and-mark-schemes/2021/november/AQA-75171-QP-NOV21.PDF>
A Harshad number is a positive integer which is exactly divisible by the sum of its
digits. The first twelve Harshad numbers are 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12 and 18
• 36 is a Harshad number. The digits of 36 are 3 and 6; the sum of these digits is 9
as 3 + 6 = 9 and 36 is exactly divisible by 9 (36 ÷ 9 = 4)
• 300 is a Harshad number. The digits of 300 are 3, 0 and 0; the sum of these digits
is 3 as 3 + 0 + 0 = 3 and 300 is exactly divisible by 3 (300 ÷ 3 = 100)
• 15 is not a Harshad number. The digits of 15 are 1 and 5; the sum of these digits is
6 as 1 + 5 = 6 and 15 is not exactly divisible by 6
Write a program that asks the user to enter a number, n, and will then calculate and
display the nth Harshad number.
You may assume that the number that the user enters will be a positive integer.
*/
pub fn get_harshad_number(_n: u64) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::harshad_numbers::get_harshad_number;

    #[test]
    fn test_harshad_number() {
        assert_eq!(18, get_harshad_number(12));
    }
}
