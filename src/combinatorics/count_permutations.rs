fn partial_factorial(start: u128, end: u128) -> u128 {
    let mut product: u128 = 1;
    for n in start..end + 1 {
        product *= n;
    }
    return product;
}

pub fn count_ways_to_pick(number: u128, set_size: u128) -> u128 {
    let mut top = partial_factorial(number + 1, set_size);
    for n in 1..set_size - number + 1 {
        top /= n;
    }
    return top;
}

#[cfg(test)]
mod partial_factorial_tests {
    use super::partial_factorial;
    #[test]
    fn partial_factorial_1_2() {
        assert_eq!(partial_factorial(1, 2), 2);
    }

    #[test]
    fn partial_factorial_1_10() {
        assert_eq!(partial_factorial(1, 10), 3628800);
    }

    #[test]
    fn partial_factorial_9_10() {
        assert_eq!(partial_factorial(9, 10), 90);
    }

    #[test]
    fn partial_factorial_900_902() {
        assert_eq!(partial_factorial(900, 902), 731431800);
    }
}

#[cfg(test)]
mod count_ways_to_pick_tests {
    use super::count_ways_to_pick;
    #[test]
    fn pick_1_from_1() {
        assert_eq!(count_ways_to_pick(1, 1), 1);
    }

    #[test]
    fn pick_1_from_3() {
        assert_eq!(count_ways_to_pick(1, 3), 3);
    }

    #[test]
    fn pick_2_from_3() {
        assert_eq!(count_ways_to_pick(2, 3), 3);
    }

    #[test]
    fn pick_2_from_4() {
        assert_eq!(count_ways_to_pick(2, 4), 6);
    }

    #[test]
    fn pick_10_from_30() {
        assert_eq!(count_ways_to_pick(10, 30), 30045015);
    }
}
