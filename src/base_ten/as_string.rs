pub fn to_string<T: num::PrimInt>(number: T) -> String {
    return String::new();
}

#[cfg(test)]
mod to_string_tests {
    use super::*;
    #[test]
    fn one_to_string() {
        assert_eq!(to_string(1), "1");
    }

    #[test]
    fn zero_to_string() {
        assert_eq!(to_string(0), "0");
    }

    #[test]
    fn ten_digits_to_string() {
        assert_eq!(to_string(1234567890), "1234567890");
    }

    #[test]
    fn something_big_to_string() {
        assert_eq!(
            to_string(1234567890_1234567890_1234567890_123456789u128),
            "123456789012345678901234567890123456789"
        );
    }
}
