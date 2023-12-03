/**
<https://filestore.aqa.org.uk/sample-papers-and-mark-schemes/2020/november/AQA-75171-QP-NOV20.PDF>
Write a program that asks the user how many numeric digits they would like to enter
and then gets the user to enter that number of numeric digits.
The program should calculate and display the number of times the most frequently entered numeric digit was input.
If more than one numeric digit had the same frequency and was the most frequently entered then instead of displaying the frequency, a message saying "Data was multimodal" should be displayed.
A numeric digit is 0, 1, 2, 3, 4, 5, 6, 7, 8 or 9
You may assume that the number that the user enters to state how many numeric digits there will be and the numeric digits entered by the user are all valid.
*/
pub fn most_frequent_digit_frequency(_digits: &[u64]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::most_frequent_digit::most_frequent_digit_frequency;

    #[test]
    fn test_most_frequent_digit() {
        assert_eq!(2, most_frequent_digit_frequency(&[3, 4, 5, 3]));
    }
}
