pub fn number_to_spoken_text(number: usize) -> String {
    match number {
        // Match a single value
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 | 16 | 17 | 19 => format!("{}teen", number_to_spoken_text(number - 10)),
        15 => String::from("fifteen"),
        18 => String::from("eighteen"),
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        60 | 70 | 90 => format!("{}ty", number_to_spoken_text(number / 10)),
        80 => String::from("eighty"),
        20..=99 => format!(
            "{} {}",
            number_to_spoken_text(number - number % 10),
            number_to_spoken_text(number % 10)
        ),
        n if n <= 999 && n % 100 == 0 => {
            format!("{} hundred", number_to_spoken_text(number / 100))
        }
        100..=999 => format!(
            "{} and {}",
            number_to_spoken_text(number - number % 100),
            number_to_spoken_text(number % 100)
        ),
        n if n <= 999999 && n % 1000 == 0 => {
            format!("{} thousand", number_to_spoken_text(number / 1000))
        }
        n if n <= 999999 && n % 1000 < 100 => format!(
            "{} and {}",
            number_to_spoken_text(number - number % 1000),
            number_to_spoken_text(number % 1000)
        ),
        1000..=999999 => format!(
            "{} {}",
            number_to_spoken_text(number - number % 1000),
            number_to_spoken_text(number % 1000)
        ),
        1000000 => String::from("one million"),
        _ => panic!("nyi"),
    }
}

#[cfg(test)]
mod test_number_to_spoken_text {
    use super::number_to_spoken_text;

    #[test]
    fn first_few() {
        assert_eq!("one", number_to_spoken_text(1));
        assert_eq!("two", number_to_spoken_text(2));
        assert_eq!("three", number_to_spoken_text(3));
        assert_eq!("four", number_to_spoken_text(4));
        assert_eq!("five", number_to_spoken_text(5));
        assert_eq!("six", number_to_spoken_text(6));
        assert_eq!("seven", number_to_spoken_text(7));
        assert_eq!("eight", number_to_spoken_text(8));
        assert_eq!("nine", number_to_spoken_text(9));
    }

    #[test]
    fn teens_irregular() {
        assert_eq!("eleven", number_to_spoken_text(11));
        assert_eq!("twelve", number_to_spoken_text(12));
        assert_eq!("thirteen", number_to_spoken_text(13));
        assert_eq!("fifteen", number_to_spoken_text(15));
        assert_eq!("eighteen", number_to_spoken_text(18));
    }

    #[test]
    fn teens_regular() {
        assert_eq!("fourteen", number_to_spoken_text(14));
        assert_eq!("sixteen", number_to_spoken_text(16));
        assert_eq!("seventeen", number_to_spoken_text(17));
        assert_eq!("nineteen", number_to_spoken_text(19));
    }

    #[test]
    fn tens() {
        assert_eq!("twenty", number_to_spoken_text(20));
        assert_eq!("thirty", number_to_spoken_text(30));
        assert_eq!("forty", number_to_spoken_text(40));
        assert_eq!("fifty", number_to_spoken_text(50));
        assert_eq!("sixty", number_to_spoken_text(60));
        assert_eq!("seventy", number_to_spoken_text(70));
        assert_eq!("eighty", number_to_spoken_text(80));
        assert_eq!("ninety", number_to_spoken_text(90));
    }

    #[test]
    fn hundreds() {
        assert_eq!("one hundred", number_to_spoken_text(100));
        assert_eq!("two hundred", number_to_spoken_text(200));
    }

    #[test]
    fn zero() {
        assert_eq!("zero", number_to_spoken_text(0));
    }

    #[test]
    fn holistic() {
        assert_eq!("forty two", number_to_spoken_text(42));
        assert_eq!("ninety nine", number_to_spoken_text(99));
        assert_eq!("three hundred and thirty three", number_to_spoken_text(333));
        assert_eq!("nine hundred and eighty seven", number_to_spoken_text(987));
        assert_eq!("one hundred and one", number_to_spoken_text(101));
        assert_eq!("nine hundred and eleven", number_to_spoken_text(911));
    }

    #[test]
    fn large() {
        assert_eq!("one thousand", number_to_spoken_text(1000));
        assert_eq!(
            "one hundred and twenty three thousand four hundred and fifty six",
            number_to_spoken_text(123456)
        );
        assert_eq!("two thousand and twenty", number_to_spoken_text(2020));
    }
}
