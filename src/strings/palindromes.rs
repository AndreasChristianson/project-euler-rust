pub fn is_palindrome(string: &str) -> bool {
    let reverse = string.chars().rev().collect::<String>();
    return reverse == string;
}

#[cfg(test)]
mod number_of_digits_tests {
    use super::is_palindrome;
    #[test]
    fn not_palindrome1() {
        assert_eq!(is_palindrome("hello"), false);
    }
    #[test]
    fn not_palindrome2() {
        assert_eq!(is_palindrome("1=I"), false);
    }
    #[test]
    fn not_palindrome3() {
        assert_eq!(is_palindrome("qwerqwer"), false);
    }
    #[test]
    fn zero_length() {
        assert_eq!(is_palindrome(""), true);
    }
    #[test]
    fn one_length() {
        assert_eq!(is_palindrome("a"), true);
    }
    #[test]
    fn odd_palindrome() {
        assert_eq!(is_palindrome("qwerewq"), true);
    }
    #[test]
    fn even_palindrome() {
        assert_eq!(is_palindrome("qwerrewq"), true);
    }
    #[test]
    fn tattarrattat() {
        assert_eq!(is_palindrome("tattarrattat"), true);
    }
}
