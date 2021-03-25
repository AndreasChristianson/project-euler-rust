const LINE_DELIMITER: char = '\n';
const TOKEN_DELIMITER: char = ' ';

pub fn parse_2d_string(string: &str) -> Vec<Vec<i32>> {
    let lines = string.trim().split(LINE_DELIMITER);
    let mut ret: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let tokens = line.split(TOKEN_DELIMITER);
        let mut numbers: Vec<i32> = Vec::new();

        for token in tokens {
            let number = token.parse::<i32>().unwrap_or(0);
            numbers.push(number);
        }
        ret.push(numbers);
    }
    return ret;
}

#[cfg(test)]
mod number_of_digits_tests {
    use super::parse_2d_string;
    #[test]
    fn one_char_string() {
        let result = parse_2d_string("1");
        assert_eq!(1, result[0][0]);
    }
    #[test]
    fn big_number_string() {
        let result = parse_2d_string("12345");
        assert_eq!(12345, result[0][0]);
    }
    #[test]
    fn jagged_string() {
        let result = parse_2d_string("1 2 3\n456\n7 8");
        assert_eq!(1, result[0][0]);
        assert_eq!(2, result[0][1]);
        assert_eq!(3, result[0][2]);
        assert_eq!(456, result[1][0]);
        assert_eq!(7, result[2][0]);
        assert_eq!(8, result[2][1]);
    }

    #[test]
    fn small_string() {
        let result = parse_2d_string("1 2 3 4");
        assert_eq!(1, result[0][0]);
        assert_eq!(2, result[0][1]);
        assert_eq!(3, result[0][2]);
        assert_eq!(4, result[0][3]);
    }
}
