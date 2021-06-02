use circular_queue::CircularQueue;

pub fn least_significant_digits(number: u128, count: usize) -> String {
    let mut queue = CircularQueue::with_capacity(count);
    let as_string = number.to_string();
    let sequence = as_string.chars();
    for n in sequence {
        queue.push(n);
    }
    return queue.iter().rev().collect::<String>();
}

#[cfg(test)]
mod number_of_digits_tests {
    use super::*;

    #[test]
    fn five_digits() {
        assert_eq!(least_significant_digits(1234567890_0987654321, 5), "54321");
    }

    #[test]
    fn short() {
        assert_eq!(least_significant_digits(11, 5), "11");
    }

    #[test]
    fn exact() {
        assert_eq!(
            least_significant_digits(1234567890_0987654321, 20),
            "12345678900987654321"
        );
    }
}
