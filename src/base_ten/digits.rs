pub fn number_of_digits(number: i128) -> usize {
    return number.abs().to_string().chars().count();
}

#[cfg(test)]
mod number_of_digits_tests {
    use super::*;
    #[test]
    fn ten_digits() {
        assert_eq!(number_of_digits(1234567890), 10);
    }

    #[test]
    fn zero() {
        assert_eq!(number_of_digits(0), 1);
    }

    #[test]
    fn negative_one() {
        assert_eq!(number_of_digits(1), 1);
    }
    #[test]
    fn negative_ten_digits() {
        assert_eq!(number_of_digits(-1234567890), 10);
    }

    #[test]
    fn big_numbers() {
        assert_eq!(
            number_of_digits(1234567890_1234567890_1234567890_123456789i128),
            39
        );
    }
}
